use std::collections::VecDeque;
use serde::{Serialize, Deserialize};

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
      time_len: len
    })
  }
  pub fn connect_two_way(&mut self, n1_id: NodeId, n2_id: NodeId, len: u32) {
    let n1 = self.node_mut(n1_id);
    n1.nodes.push(NodeLink {
      node: n2_id,
      time_len: len
    });
    let n2 = self.node_mut(n2_id);
    n2.nodes.push(NodeLink {
      node: n1_id,
      time_len: len
    });
  }

  pub fn shortest_path(&mut self, start: NodeId, end: NodeId) {
    let mut queue = VecDeque::new();
    self.node_mut(start).eta = 0;
    queue.push_back(start);
    while let Some(node_id) = queue.pop_back() {
      let node = self.node(node_id);
      for link in node.nodes.iter() {
        let next_node = self.node_mut(link.node);
        if node.eta + link.time_len < next_node.eta {
          next_node.eta = node.eta + link.time_len;
          queue.push_back(link.node);
        }
      }
    }
  }
}

#[derive(Copy, Clone, Serialize, Debug)]
pub struct NodeId(usize);

#[derive(Serialize)]
pub struct Node {
  pub nodes: Vec<NodeLink>,
  pub eta: u32,
  pub kind: NodeKind
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
  time_len: u32,
}