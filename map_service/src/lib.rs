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

// #[pyclass]
// #[derive(Debug)]
// pub struct MapCarPaths {
//   paths: Vec<Vec<MapPoint>>
// }

#[pyclass]
#[derive(Debug)]
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
    println!("useless nodes: {}/{}", cnt, self.nodes.len());
    self.build_graph();
  }

  /// build graph for humans
  fn build_graph(&mut self) {
    let mut node_id_map = HashMap::new();
    for node in self.nodes.values() {
      let id = self.graph.add_node(Node {
        nodes: Vec::new(),
        eta: u32::MAX,
        kind: NodeKind::Plain,
        lat: node.lat,
        lon: node.lon
      }, node.id);
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

  pub fn build_path_using_cars(&mut self, points: Vec<PyRef<MapPoint>>, car_paths: &PyList) -> PyResult<Vec<MapPoint>> {
    let mut paths = Vec::new();
    for l in car_paths.iter().map(|a| a.extract::<&PyList>()) {
      let mut points = Vec::new();
      for point in l?.iter().map(|v| v.extract::<&MapPoint>()) {
        points.push(point?);
      }
      paths.push(points);
    }

    Ok(Vec::new())
  }
}

impl MapService {
  pub fn build_path_rust(&mut self, points: Vec<&MapPoint>) -> Vec<MapPoint> {
    #[derive(Clone)]
    struct ClosestNode {
      id: u64,
      dist: f64
    };
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
        self.graph.shortest_path(prev, curr).into_iter()
            .map(|id| MapPoint::from(self.nodes.get(&id).unwrap()))
            .skip(1)
      );
      prev = curr;
    }
    let en = std::time::Instant::now();
    println!("Build path in {}s.", (en - st).as_secs_f64());
    path
  }
}

/// MapService responsible for working with map data and paths.
#[pymodule]
fn map_service(_py: Python, m: &PyModule) -> PyResult<()> {
  m.add_class::<MapService>()?;
  m.add_class::<MapPoint>()?;
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
