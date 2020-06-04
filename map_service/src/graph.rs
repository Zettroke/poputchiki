use std::collections::{VecDeque, BinaryHeap, HashMap};
use serde::{Serialize, Deserialize};
use std::cmp::Ordering;
use crate::distance;
use std::ops::Deref;

#[derive(Serialize)]
pub struct RoadGraph {
  pub nodes: Vec<Node>,
  /// OSM'овские id нод. Прямое соответствие
  pub osm_nodes_ids: Vec<u64>
}

impl RoadGraph {
  pub fn new() -> Self {
    Self {
      nodes: Vec::new(),
      osm_nodes_ids: Vec::new()
    }
  }
  pub fn node<'a, 'b>(&'a self, id: NodeId) -> &'b Node {
    unsafe { std::mem::transmute::<_, &'b Node>(self.nodes.get(id.0).unwrap()) }
  }
  pub fn node_mut<'a, 'b>(&'a mut self, id: NodeId) -> &'b mut Node {
    unsafe { std::mem::transmute::<_, &'b mut Node>(self.nodes.get_mut(id.0).unwrap()) }
  }

  pub fn add_node(&mut self, node: Node, osm_id: u64) -> NodeId {
    self.nodes.push(node);
    self.osm_nodes_ids.push(osm_id);
    NodeId(self.nodes.len() - 1)
  }

  pub fn connect_one_way(&mut self, n1_id: NodeId, n2_id: NodeId, len: u32) {
    let n1 = self.node_mut(n1_id);
    n1.nodes.push(NodeLink {
      node: n2_id,
      len: len
    })
  }
  pub fn connect_two_way(&mut self, n1_id: NodeId, n2_id: NodeId, len: u32) {
    let n1 = self.node_mut(n1_id);
    n1.nodes.push(NodeLink {
      node: n2_id,
      len: len
    });
    let n2 = self.node_mut(n2_id);
    n2.nodes.push(NodeLink {
      node: n1_id,
      len: len
    });
  }

  pub fn osm_id(&self, id: NodeId) -> u64 {
    *self.osm_nodes_ids.get(id.0).unwrap()
  }

  pub fn node_id_by_osm_id(&self, id: u64) -> Option<NodeId> {
    self.osm_nodes_ids.iter()
      .position(|osm_id| *osm_id == id)
      .map(|id| NodeId(id))
  }

  fn reset_graph(&mut self) {
    for n in self.nodes.iter_mut() {
      n.eta = u32::MAX;
    }
  }

  pub fn shortest_path(&mut self, start: NodeId, end: NodeId) -> Vec<u64> {
    self.reset_graph();
    let mut queue = BinaryHeap::new();
    self.node_mut(start).eta = 0;
    let start_node = self.node(start);
    let end_node = self.node(end);
    queue.push(State { cost: start_node.eta + distance(start_node, end_node), node: start });
    while let Some(state) = queue.pop() {
      if state.node == end {
        println!("queue len = {}", queue.len());
        println!("dist = {}", self.node(state.node).eta);
        break;
      }
      let node = self.node(state.node);
      for link in node.nodes.iter() {
        let next_node = self.node_mut(link.node);
        if next_node.eta > node.eta + link.len {
          next_node.eta = node.eta + link.len;
          let dist = distance(next_node, end_node);
          queue.push(State { cost: next_node.eta + dist, node: link.node });
        }
      }
    }
    if end_node.eta == u32::MAX {
      return  Vec::new();
    } else {
      let mut path = vec![self.osm_id(end)];

      let mut curr_node = end_node;
      'main: while curr_node.eta != 0 {
        for link in curr_node.nodes.iter() {
          let n = self.node(link.node);
          if n.eta == curr_node.eta.overflowing_sub(link.len).0 {
            curr_node = n;
            path.push(self.osm_id(link.node));
            continue 'main;
          }
        }
        println!("Couldn't find path");
        break;
      }
      path.reverse();
      return path;
    }
  }
}

struct State {
  cost: u32,
  node: NodeId
}

impl Ord for State {
  fn cmp(&self, other: &Self) -> Ordering {
    self.cost.cmp(&other.cost).reverse()
  }
}

impl Eq for State {}
impl PartialEq for State {
  fn eq(&self, other: &Self) -> bool {
    self.cost.eq(&other.cost)
  }
}
impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.cost.partial_cmp(&other.cost).map(|o| o.reverse())
  }
}
#[derive(Copy, Clone, Serialize, Debug, PartialEq, Eq)]
pub struct NodeId(usize);

#[derive(Serialize)]
pub struct Node {
  pub nodes: Vec<NodeLink>,
  pub eta: u32,
  pub kind: NodeKind,
  pub lon: f64,
  pub lat: f64
}

#[derive(Serialize)]
pub enum NodeKind {
  Plain,
  Car {
    eta: u32,
    free_seats: u8
  }
}
#[derive(Serialize)]
pub struct NodeLink {
  node: NodeId,
  len: u32,
}