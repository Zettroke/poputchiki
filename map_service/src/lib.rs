use pyo3::prelude::*;
use std::collections::HashMap;
use crate::osm_map::{OsmNode, OsmWay};
use std::fs::File;
use std::io::{BufReader, Write};
use flate2::bufread::GzDecoder;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use crate::graph::{RoadGraph, Node, NodeKind};
use std::rc::Rc;
use pyo3::types::{PyDict, PyList};
use pyo3::exceptions::TypeError;
use std::borrow::Borrow;
use std::ops::Deref;
use pyo3::{PyGCProtocol, PyVisit, PyTraverseError};
use serde::Serialize;

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
#[derive(Debug)]
pub struct MapCarPath {
  #[pyo3(get)]
  start_at: i64,
  #[pyo3(get)]
  path: Vec<PyObject>
}

#[pymethods]
impl MapCarPath {
  #[new]
  pub fn new(start_at: i64, path: Vec<PyObject>) -> Self {
    Self {
      start_at,
      path
    }
  }
}

impl MapCarPath {
  pub fn points<'a>(&'a self, py: Python<'a>) -> PyResult<Vec<PyRef<MapPoint>>> {
    let mut res = Vec::new();
    for o in self.path.iter() {
      res.push(o.extract::<PyRef<MapPoint>>(py)?);
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

  pub fn to_json<'a>(&self, py: Python<'a>) -> &'a PyDict {
    let mut d = PyDict::new(py);
    d.set_item("id", self.id);
    d.set_item("lat", self.lat);
    d.set_item("lon", self.lon);
    if let Some(path_id) = self.path_id {
      d.set_item("path_id", path_id);
    }
    d
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

    // let cnt = self.nodes.values().filter(|v| Rc::strong_count(&v.0) == 2).count();
    // warn!("useless nodes: {}/{}", cnt, self.nodes.len());
    self.build_graph();
  }

  /// build graph for humans
  fn build_graph(&mut self) {
    let mut node_id_map = HashMap::new();
    for node in self.nodes.values() {
      let id = self.graph.add_node(Node {
        nodes: Vec::new(),
        eta: u32::MAX,
        id: node.id,
        kind: NodeKind::Plain,
        lat: node.lat,
        lon: node.lon
      });
      node_id_map.insert(node.id, id);
    }

    for way in self.ways.values() {
      let mut prev_node_id = *node_id_map.get(&way.nodes[0].id).unwrap();
      for node in &way.nodes[1..] {
        let curr_node_id = *node_id_map.get(&node.id).unwrap();
        self.graph.connect_two_way(
          prev_node_id,
          curr_node_id,
          distance_t(self.graph.node(prev_node_id), self.graph.node(curr_node_id), Kmh(5))
        );
        prev_node_id = curr_node_id;
      }
    }
  }

  pub fn build_path(&mut self, points: Vec<PyRef<MapPoint>>) -> Vec<MapPoint> {
    self.build_path_rust(points.iter().map(|p| p.deref()).collect())
  }

  pub fn build_path_using_cars(&mut self, py: Python, start_at: i64, points: Vec<PyRef<MapPoint>>, car_paths: Vec<PyRef<MapCarPath>>) -> PyResult<Vec<MapPoint>> {
    let points: Vec<&MapPoint> = points.iter().map(|p| p.deref()).collect();
    let mut v = Vec::new();
    let mut tmp = Vec::new();
    for p in car_paths.iter() {
      tmp.push(p.points(py.clone())?);
    }
    for a in tmp.iter() {
      let pmcp = PlainMapCarPath {
        start_at,
        path: a.iter().map(|v| v.deref()).collect()
      };
      v.push(pmcp);
    }

    Ok(self.build_path_using_cars_rust(start_at, points, v))
  }
}

#[derive(Clone)]
struct ClosestNode {
  id: u64,
  dist: f64
}

impl MapService {
  pub fn build_path_rust(&mut self, points: Vec<&MapPoint>) -> Vec<MapPoint> {
    let st = std::time::Instant::now();
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

    let start_node_id = closest.get(0).unwrap().id;
    let mut path = vec![MapPoint::from(self.nodes.get(&start_node_id).unwrap())];
    let mut prev = self.graph.node_id_by_osm_id(start_node_id).unwrap();

    for cl in closest.iter().skip(1) {
      let curr = self.graph.node_id_by_osm_id(cl.id).unwrap();
      path.extend(
        self.graph.shortest_path(prev, curr).into_iter().skip(1)
      );
      prev = curr;
    }
    let en = std::time::Instant::now();
    info!("Build path in {}s.", (en - st).as_secs_f64());
    path
  }

  pub fn build_path_using_cars_rust(&mut self, start_at: i64, points: Vec<&MapPoint>, car_paths: Vec<PlainMapCarPath>) -> Vec<MapPoint> {
    let st = std::time::Instant::now();
    for p in car_paths.iter() {
      let ppoints = &p.path;

      let mut prev_point = ppoints.first().unwrap().deref();
      let mut prev_car_eta = p.start_at - start_at;
      let mut prev_node_id = self.graph.add_car_map_point(prev_point, 255);
      self.graph.set_node_eta(prev_node_id, prev_car_eta);
      self.graph.connect_two_way(
        prev_node_id,
        *self.graph.node_map.get(&prev_point.id).unwrap(),
        1
      );

      for curr_point in ppoints.iter().skip(1) {
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
          1
        );
        prev_car_eta = curr_car_eta;
        prev_node_id = curr_node_id;
        prev_point = curr_point;
      }
    }

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
