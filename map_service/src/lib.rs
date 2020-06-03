use pyo3::prelude::*;
use std::collections::HashMap;
use crate::osm_map::{OsmNode, OsmWay};
use std::fs::File;
use std::io::{BufReader, Write};
use flate2::bufread::GzDecoder;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use crate::utils::{u64_parse, f64_parse};
use crate::graph::{RoadGraph, Node, NodeKind};
use std::rc::Rc;
use pyo3::types::PyDict;

pub mod osm_map;
pub mod graph;
pub mod utils;

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

#[pyclass]
#[derive(Debug)]
pub struct MapPoint {
  #[pyo3(get)]
  pub id: u64,
  #[pyo3(get)]
  pub lat: f64,
  #[pyo3(get)]
  pub lon: f64
}

#[pymethods]
impl MapPoint {
  #[new]
  pub fn new(lat: f64, lon: f64) -> Self {
    Self {
      id: 0,
      lat,
      lon
    }
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
      lon: n.lon
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
    let reader = BufReader::new(GzDecoder::new(BufReader::new(File::open(path).unwrap())));
    let mut event_reader = Reader::from_reader(reader);
    let mut buf = Vec::new();
    let mut nodes = HashMap::new();
    let mut ways = HashMap::new();

    let mut current_way: Option<OsmWay> = None;
    let mut is_current_way_highway = false;
    loop {
      match event_reader.read_event(&mut buf) {
        Ok(Event::Start(ref e)) => {
          match e.name() {
            b"node" => {
              let node = handle_node(e);
              nodes.insert(node.id, node);
            },
            b"way" => {
              let mut id = 0u64;
              e.attributes().find(|v| {
                v.as_ref().map_or(false, |vv| vv.key == b"id")
              }).map(|res| id = u64_parse(res.unwrap().value.as_ref()));
              current_way = Some(OsmWay::new(id, "".to_string()));
            }
            _ => {}
          }
        },
        Ok(Event::End(ref e)) => {
          match e.name() {
            b"way" => {
              if let Some(way) = current_way.take() {
                if is_current_way_highway {
                  ways.insert(way.id, way);
                  is_current_way_highway = false;
                }
              }
            },
            _ => {}
          }
        },
        Ok(Event::Empty(ref e)) => {
          match e.name() {
            b"node" => {
              let node = handle_node(e);
              nodes.insert(node.id, node);
            },
            b"nd" => {
              let nd_ref =
                e.attributes().find(|a| a.as_ref().map_or(false, |a| a.key == b"ref"))
                  .map(|v| u64_parse(v.unwrap().value.as_ref()));
              if let Some(ref nd_id) = nd_ref {
                if let Some(node) = nodes.get(nd_id) {
                  current_way.as_mut().map(|w| w.nodes.push(node.clone()));
                }
              }
            },
            b"tag" => {
              if let Some(ref mut way) = current_way {
                let mut is_current_tag_highway = false;
                e.attributes().for_each(|attr| {
                  attr.map(|a| {
                    match a.key {
                      b"k" => { is_current_tag_highway = a.value.as_ref() == b"highway" },
                      b"v" => {
                        if is_current_tag_highway {
                          is_current_way_highway = true;
                          way.highway_type = String::from_utf8(a.value.to_vec()).unwrap();
                        }
                      },
                      _ => {}
                    }
                  }).unwrap();
                });
              }
            }
            _ => {}
          }
        },
        Ok(Event::Eof) => break,
        Err(e) => panic!("{:?}", e),
        _ => {}
      }
    }
    self.nodes = nodes;
    self.nodes.retain(|_, v| Rc::strong_count(&v.0) > 1);
    self.ways = ways;
    let cnt = self.nodes.values().filter(|v| Rc::strong_count(&v.0) == 2).count();
    println!("useless nodes: {}/{}", cnt, self.nodes.len());
    self.build_graph();
  }

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
          distance(self.graph.node(prev_node_id), self.graph.node(curr_node_id))
        );
        prev_node_id = curr_node_id;
      }
    }
  }

  pub fn build_path(&mut self, points: Vec<PyRef<MapPoint>>) -> Vec<MapPoint> {
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


fn handle_node(e: &BytesStart) -> OsmNode {
  let mut id = 0; let mut lat = 0.0; let mut lon = 0.0;
  e.attributes().for_each(|v| {
    if let Ok(a) = v {
      match a.key {
        b"id" => {
          id = u64_parse(a.value.as_ref());
        },
        b"lat" => {
          lat = f64_parse(a.value.as_ref());
        },
        b"lon" => {
          lon = f64_parse(a.value.as_ref());
        },
        _ => {}
      }
    }
  });

  return OsmNode::new(id, lat, lon);
}

/// MapService responsible for working with map data and paths.
#[pymodule]
fn map_service(_py: Python, m: &PyModule) -> PyResult<()> {
  // m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
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
