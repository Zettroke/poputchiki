use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;
use crate::osm_map::{OsmNode, OsmWay};
use std::fs::File;
use std::io::BufReader;
use flate2::bufread::GzDecoder;
use xml::EventReader;
use xml::reader::XmlEvent;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use std::str::FromStr;
use std::borrow::BorrowMut;
use crate::utils::{u64_parse, f64_parse};

pub mod osm_map;
pub mod graph;
pub mod utils;

#[pyclass]
pub struct MapService {
    pub nodes: Vec<OsmNode>,
    pub ways: HashMap<u64, OsmWay>
}

#[pymethods]
impl MapService {
    #[new]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            ways: HashMap::new()
        }
    }

    pub fn load(&mut self, path: String) {
        let mut reader = BufReader::new(GzDecoder::new(BufReader::new(File::open(path).unwrap())));
        let mut event_reader = Reader::from_reader(reader);
        let mut buf = Vec::new();
        let mut nodes = HashMap::new();
        let mut ways = HashMap::new();
        let mut current_way: Option<OsmWay> = None; // OsmWay::new(std::u64::MAX);
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
                            current_way = Some(OsmWay::new(id));
                        }
                        _ => {}
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"way" => {
                            current_way.take().map(|v| ways.insert(v.id, v));
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
                        }
                        _ => {}
                    }
                },
                Ok(Event::Eof) => break,
                Err(e) => panic!("{:?}", e),
                _ => {}
            }
        }
        self.nodes = nodes.drain().map(|v| v.1).collect();
        self.nodes.sort_unstable_by_key(|n| n.id);
        self.ways = ways;
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
fn map_service(py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    m.add_class::<MapService>()?;
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
