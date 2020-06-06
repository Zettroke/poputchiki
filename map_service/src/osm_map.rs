use std::rc::Rc;
use std::ops::{Deref, DerefMut};
use crate::utils::{u64_parse, f64_parse};
use quick_xml::events::{Event, BytesStart};
use std::io::BufReader;
use flate2::read::GzDecoder;
use std::fs::File;
use quick_xml::Reader;
use std::collections::HashMap;

pub struct OsmNode(pub Rc<InnerNode>);

impl Deref for OsmNode {
  type Target = InnerNode;

  fn deref(&self) -> &Self::Target {
    self.0.deref()
  }
}
impl Clone for OsmNode {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

pub struct InnerNode {
  pub id: u64,
  pub lat: f64,
  pub lon: f64
}

impl OsmNode {
  pub fn new(id: u64, lat: f64, lon: f64) -> Self {
    OsmNode(Rc::new(InnerNode {
      id,
      lat,
      lon
    }))
  }
}

pub struct OsmWay(Rc<InnerWay>);

impl Deref for OsmWay {
  type Target = InnerWay;

  fn deref(&self) -> &Self::Target {
    self.0.deref()
  }
}

impl DerefMut for OsmWay {
  fn deref_mut(&mut self) -> &mut Self::Target {
    Rc::get_mut(&mut self.0)
      .expect("This Way is already referenced by someone!")
  }
}

impl OsmWay {
  pub fn new(id: u64, s: String) -> Self {
    Self(
      Rc::new(InnerWay {
        id,
        nodes: Vec::new(),
        highway_type: s
      })
    )
  }
}

pub struct InnerWay {
  pub id: u64,
  pub nodes: Vec<OsmNode>,
  pub highway_type: String
}

pub fn load(path: String) -> (HashMap<u64, OsmNode>, HashMap<u64, OsmWay>) {
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
            if let Some(res) = e.attributes().find(|v| {
              v.as_ref().map_or(false, |vv| vv.key == b"id")
            }) {
              id = u64_parse(res.unwrap().value.as_ref());
            }

            current_way = Some(OsmWay::new(id, "".to_string()));
          }
          _ => {}
        }
      },
      Ok(Event::End(ref e)) => {
        if e.name() == b"way" {
          if let Some(way) = current_way.take() {
            if is_current_way_highway {
              ways.insert(way.id, way);
              is_current_way_highway = false;
            }
          }
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
                if let Some(w) = current_way.as_mut() { w.nodes.push(node.clone()) }
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
  nodes.retain(|_, v| Rc::strong_count(&v.0) > 1);
  // self.ways = ways;
  // let cnt = self.nodes.values().filter(|v| Rc::strong_count(&v.0) == 2).count();
  // println!("useless nodes: {}/{}", cnt, self.nodes.len());
  // self.build_graph();
  (nodes, ways)
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

  OsmNode::new(id, lat, lon)
}
