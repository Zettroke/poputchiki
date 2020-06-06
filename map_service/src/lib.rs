use pyo3::prelude::*;
use std::collections::HashMap;
use crate::osm_map::{OsmNode, OsmWay};
use crate::graph::{RoadGraph, Node, NodeKind, ROAD_TO_CAR};
use pyo3::types::PyDict;
use std::ops::Deref;
use pyo3::{PyGCProtocol, PyVisit, PyTraverseError};
use serde::Serialize;
use std::rc::Rc;

#[macro_use] extern crate log;

pub mod osm_map;
pub mod graph;
pub mod utils;

/// distance in centimeters
pub fn distance(n1: &Node, n2: &Node) -> u32 {
  let lat1 = n1.lat.to_radians();
  let lat2 = n2.lat.to_radians();
  let lon1 = n1.lon.to_radians();
  let lon2 = n2.lon.to_radians();
  let dlat = lat2 - lat1;
  let dlon = lon2 - lon1;

  let a = (dlat/2.).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon/2.).sin().powi(2);
  let c = 2. * (a.sqrt()).atan2((1.-a).sqrt());

  return (c * 6_371_302_00.0).round() as u32; // cm
}

/// distance in milliseconds
pub fn distance_t(n1: &Node, n2: &Node, speed: Kmh) -> u32 {
  let cm = distance(n1, n2);
  (cm as f64 / speed.as_cm_per_millisecond()).round() as u32
}

/// Km/h
pub struct Kmh(u32);
impl From<u32> for Kmh {
  fn from(v: u32) -> Self {
    Self(v)
  }
}
impl Kmh{
  pub fn as_cm_per_millisecond(&self) -> f64 {
    self.0 as f64 / 3.6 / 1000.0 * 100.0
  }
}

pub struct PlainMapCarPath<'a> {
  pub start_at: i64,
  pub path: Vec<&'a MapPoint>
}

#[pyclass]
#[derive(Debug, Default)]
pub struct PathResultObject {
  #[pyo3(get)]
  pub total_time: u32,
  #[pyo3(get)]
  pub points: Vec<Py<MapPoint>>,
  #[pyo3(get)]
  pub eta_list: Vec<u32>
}
#[pymethods]
impl PathResultObject {
  pub fn to_json<'a>(&self, py: Python<'a>) -> PyResult<&'a PyDict> {
    let d = PyDict::new(py);

    d.set_item("total_time", self.total_time)?;
    d.set_item("points", &self.points.iter().map(|p| {
      let v = (p.as_ref(py) as &PyCell<MapPoint>).borrow();
      v.to_json(py.clone())
    }).collect::<PyResult<Vec<&PyDict>>>()?)?;
    d.set_item("eta_list", &self.eta_list)?;

    Ok(d)
  }
}

impl PathResultObject {
  pub fn from_path_result(py: Python, pr: PathResult) -> Self {
    Self {
      total_time: pr.total_time,
      eta_list: pr.eta_list,
      points: pr.points.into_iter().map(|p| Py::new(py, p).unwrap()).collect()
    }
  }
}

#[derive(Debug, Default, Serialize)]
pub struct PathResult {
  pub total_time: u32,
  pub points: Vec<MapPoint>,
  pub eta_list: Vec<u32>
}

#[pyclass]
#[derive(Debug)]
pub struct MapCarPath {
  #[pyo3(get)]
  start_at: i64,
  #[pyo3(get)]
  path: Vec<Py<MapPoint>>
}

#[pymethods]
impl MapCarPath {
  #[new]
  pub fn new(start_at: i64, path: Vec<Py<MapPoint>>) -> Self {
    Self {
      start_at,
      path
    }
  }
}

impl MapCarPath {
  pub fn points<'a>(&'a self, py: Python<'a>) -> PyResult<Vec<PyRef<MapPoint>>> {
    let mut res: Vec<PyRef<MapPoint>> = Vec::new();
    for o in self.path.iter() {
      res.push((o.as_ref(py) as &PyCell<MapPoint>).borrow());
    }

    Ok(res)
  }
}

#[pyproto]
impl PyGCProtocol for MapCarPath {
  fn __traverse__(&self, visit: PyVisit) -> Result<(), PyTraverseError> {
    for o in self.path.iter() {
      visit.call(o)?;
    }

    Ok(())
  }

  fn __clear__(&mut self) {
    let gil = GILGuard::acquire();
    let py = gil.python();
    for o in self.path.drain(..) {
      py.release(o);
    }
  }
}

#[pyclass]
#[derive(Debug, Serialize)]
pub struct MapPoint {
  #[pyo3(get)]
  pub id: u64,
  #[pyo3(get)]
  pub lat: f64,
  #[pyo3(get)]
  pub lon: f64,
  #[pyo3(get)]
  pub path_id: Option<u64>
}

#[pymethods]
impl MapPoint {
  #[new]
  #[args(kwargs="**")]
  pub fn new(id: u64, lat: f64, lon: f64, kwargs: Option<&PyDict>) -> PyResult<Self> {
    let path_id = kwargs.map(|d|
        d.get_item("path_id")
            .map(|v| v.extract::<Option<u64>>())
            .unwrap_or(Ok(None))
    ).unwrap_or(Ok(None));

    Ok(Self {
      id,
      lat,
      lon,
      path_id: path_id?
    })
  }

  pub fn to_json<'a>(&self, py: Python<'a>) -> PyResult<&'a PyDict> {
    let d = PyDict::new(py);
    d.set_item("id", self.id)?;
    d.set_item("lat", self.lat)?;
    d.set_item("lon", self.lon)?;
    if let Some(path_id) = self.path_id {
      d.set_item("path_id", path_id)?;
    }

    Ok(d)
  }
}

impl From<&OsmNode> for MapPoint {
  fn from(n: &OsmNode) -> Self {
    MapPoint {
      id: n.id,
      lat: n.lat,
      lon: n.lon,
      path_id: None
    }
  }
}
impl From<&Node> for MapPoint {
  fn from(n: &Node) -> Self {
    MapPoint {
      id: n.id,
      lat: n.lat,
      lon: n.lon,
      path_id: if let NodeKind::Car {..} = n.kind {Some(1)} else {None}
    }
  }
}

#[pyclass]
pub struct MapService {
  pub nodes: HashMap<u64, OsmNode>,
  pub ways: HashMap<u64, OsmWay>,
  pub graph: RoadGraph
}

#[pymethods]
impl MapService {
  #[new]
  pub fn new() -> Self {
    Self {
      nodes: HashMap::new(),
      ways: HashMap::new(),
      graph: RoadGraph::new()
    }
  }

  pub fn load(&mut self, path: String) {
    let (nodes, ways) = crate::osm_map::load(path);
    self.nodes = nodes;
    self.ways = ways;

    let cnt = self.nodes.values().filter(|v| Rc::strong_count(&v.0) == 2).count();
    warn!("useless nodes: {}/{}", cnt, self.nodes.len());
    self.build_graph();
  }

  fn build_graph(&mut self) {
    for node in self.nodes.values() {
      self.graph.add_node(Node {
        nodes: Vec::new(),
        eta: u32::MAX,
        id: node.id,
        kind: NodeKind::Plain,
        lat: node.lat,
        lon: node.lon
      });
    };

    for way in self.ways.values() {
      let mut prev_node_id = *self.graph.node_map.get(&way.nodes[0].id).unwrap();

      for node in &way.nodes[1..] {
        let curr_node_id = *self.graph.node_map.get(&node.id).unwrap();
        self.graph.connect_two_way(
          prev_node_id,
          curr_node_id,
          distance_t(self.graph.node(prev_node_id), self.graph.node(curr_node_id), Kmh(5))
        );

        prev_node_id = curr_node_id;
      }
    }

    self.graph.nodes.iter_mut().for_each(|n| n.nodes.shrink_to_fit());
  }

  pub fn build_path(&mut self, py: Python, points: Vec<PyRef<MapPoint>>) -> PathResultObject {
    let pr = self.build_path_rust(points.iter().map(|p| p.deref()).collect());

    PathResultObject::from_path_result(py, pr)
  }

  pub fn build_path_using_cars(&mut self, py: Python, start_at: i64, points: Vec<PyRef<MapPoint>>, car_paths: Vec<PyRef<MapCarPath>>) -> PyResult<PathResultObject> {
    let points: Vec<&MapPoint> = points.iter().map(|p| p.deref()).collect();
    let mut plain_car_paths = Vec::new();

    let tmp: Vec<Vec<PyRef<MapPoint>>> = car_paths.iter()
        .map(|p| p.points(py.clone()))
        .collect::<PyResult<Vec<_>>>()?;

    for a in tmp.iter() {
      let pmcp = PlainMapCarPath {
        start_at,
        path: a.iter().map(|v| v.deref()).collect()
      };
      plain_car_paths.push(pmcp);
    }

    Ok(PathResultObject::from_path_result(py, self.build_path_using_cars_rust(start_at, points, plain_car_paths)))
  }
}

#[derive(Clone)]
struct ClosestNode {
  id: u64,
  dist: f64
}

impl MapService {
  pub fn build_path_rust(&mut self, points: Vec<&MapPoint>) -> PathResult {
    let st = std::time::Instant::now();

    let closest = self.get_closest_list(points);

    let start_node_id = closest.get(0).unwrap().id;
    let mut prev = self.graph.node_id_by_osm_id(start_node_id).unwrap();
    let mut path_result = PathResult {
      points: vec![MapPoint::from(self.nodes.get(&start_node_id).unwrap())],
      eta_list: vec![0],
      total_time: 0
    };

    for cl in closest.iter().skip(1) {
      let curr = self.graph.node_id_by_osm_id(cl.id).unwrap();
      let pr = self.graph.shortest_path(prev, curr);
      let prev_total_time = path_result.total_time;

      path_result.points.extend(
        pr.points.into_iter().skip(1)
      );
      path_result.eta_list.extend(
        pr.eta_list.into_iter().skip(1).map(|t| prev_total_time + t)
      );
      path_result.total_time = prev_total_time + pr.total_time;

      prev = curr;
    }

    let en = std::time::Instant::now();
    info!("Build path in {}s.", (en - st).as_secs_f64());
    path_result
  }

  fn get_closest_list(&self, points: Vec<&MapPoint>) -> Vec<ClosestNode> {
    let mut closest = vec![ClosestNode { id: 0, dist: f64::MAX }; points.len()];

    for (k, v) in self.nodes.iter() {
      for (ind, point) in points.iter().enumerate() {
        let mut cl = &mut closest[ind];
        let d = (v.lat - point.lat).powi(2) + (v.lon - point.lon).powi(2);
        if cl.dist > d {
          cl.dist = d;
          cl.id = *k;
        }
      }
    }

    closest
  }

  pub fn build_path_using_cars_rust(&mut self, start_at: i64, points: Vec<&MapPoint>, car_paths: Vec<PlainMapCarPath>) -> PathResult {
    let st = std::time::Instant::now();
    for p in car_paths.iter() {
      let first_point = p.path.first().unwrap().deref();
      let mut prev_car_eta = p.start_at - start_at;
      let mut prev_node_id = self.graph.add_car_map_point(first_point, 255);

      self.graph.set_node_eta(prev_node_id, prev_car_eta);
      self.graph.connect_two_way(
        prev_node_id,
        *self.graph.node_map.get(&first_point.id).unwrap(),
        ROAD_TO_CAR
      );

      for curr_point in p.path.iter().skip(1) {
        let curr_node_id = self.graph.add_car_map_point(curr_point, 255);

        let curr_car_eta = prev_car_eta + distance_t(
          self.graph.node(prev_node_id),
          self.graph.node(curr_node_id),
          Kmh(50)
        ) as i64;

        // connect to prev TODO: connect one way
        self.graph.connect_two_way(
          curr_node_id,
          prev_node_id,
          (curr_car_eta - prev_car_eta) as u32
        );

        self.graph.set_node_eta(curr_node_id, curr_car_eta);
        // connect to road node
        self.graph.connect_two_way(
          curr_node_id,
          *self.graph.node_map.get(&curr_point.id).unwrap(),
          ROAD_TO_CAR
        );

        prev_car_eta = curr_car_eta;
        prev_node_id = curr_node_id;
      }
    }

    let closest = self.get_closest_list(points);
    let n1 = *self.graph.node_map.get(&closest[0].id).unwrap();
    let n2 = *self.graph.node_map.get(&closest[1].id).unwrap();

    let res = self.graph.shortest_path(n1, n2);

    let en = std::time::Instant::now();
    info!("Build path in {}s.", (en - st).as_secs_f64());

    res
  }
}

/// MapService responsible for working with map data and paths.
#[pymodule]
fn map_service(_py: Python, m: &PyModule) -> PyResult<()> {
  if env_logger::try_init().is_ok() {
    warn!("LOGGER INITED");
  }

  m.add_class::<MapService>()?;
  m.add_class::<MapPoint>()?;
  m.add_class::<MapCarPath>()?;

  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::sum_as_string;

  #[test]
  fn it_works() {
    assert_eq!(sum_as_string(452,785).unwrap(), (452 + 785).to_string());
  }
}
