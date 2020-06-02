use std::rc::Rc;
use std::ops::{Deref, DerefMut};

pub struct OsmNode(pub Rc<InnerNode>);

impl Deref for OsmNode {
  type Target = InnerNode;

  fn deref(&self) -> &Self::Target {
    self.0.deref()
  }
}
impl Clone for OsmNode {
  fn clone(&self) -> Self {
    return Self(self.0.clone())
  }
}

pub struct InnerNode {
  pub id: u64,
  pub lat: f64,
  pub lon: f64
}

impl OsmNode {
  pub fn new(id: u64, lat: f64, lon: f64) -> Self {
    return OsmNode(Rc::new(InnerNode {
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
