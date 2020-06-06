use std::collections::{BinaryHeap, HashMap};
use serde::Serialize;
use std::cmp::Ordering;
use crate::{MapPoint, distance_t, Kmh, PathResult};

pub const ROAD_TO_CAR: u32 = 1000;

#[derive(Serialize, Default)]
pub struct RoadGraph {
  pub node_map: HashMap<u64, NodeId>,
  pub nodes: Vec<Node>,
  pub additional_nodes_num: u32,
}

impl RoadGraph {
  pub fn new() -> Self {
    Self::default()
  }
  pub fn node<'a, 'b>(&'a self, id: NodeId) -> &'b Node {
    unsafe { std::mem::transmute::<_, &'b Node>(self.nodes.get(id.0).unwrap()) }
  }
  pub fn node_mut<'a, 'b>(&'a mut self, id: NodeId) -> &'b mut Node {
    unsafe { std::mem::transmute::<_, &'b mut Node>(self.nodes.get_mut(id.0).unwrap()) }
  }

  pub fn add_node(&mut self, node: Node) -> NodeId {
    let id = NodeId(self.nodes.len());

    self.node_map.insert(node.id, id);
    self.nodes.push(node);

    id
  }

  pub fn add_map_point(&mut self, p: &MapPoint) -> NodeId {
    let id = NodeId(self.nodes.len());

    self.node_map.insert(p.id, id);
    self.nodes.push(Node {
      id: p.id,
      lat: p.lat,
      lon: p.lon,
      eta: u32::MAX,
      kind: NodeKind::Plain,
      nodes: Vec::new()
    });

    id
  }

  pub fn add_car_map_point(&mut self, p: &MapPoint, free_seats: u8) -> NodeId {
    self.additional_nodes_num += 1;
    self.nodes.push(Node {
      id: p.id,
      lat: p.lat,
      lon: p.lon,
      eta: u32::MAX,
      kind: NodeKind::Car { eta: 0, free_seats },
      nodes: Vec::new()
    });

    NodeId(self.nodes.len() - 1)
  }

  pub fn set_node_eta(&mut self, id: NodeId, eta: i64) {
    if let NodeKind::Car { eta: ref mut orig_eta, .. } = self.node_mut(id).kind {
      *orig_eta = eta;
    }
  }

  pub fn connect_two_way(&mut self, n1_id: NodeId, n2_id: NodeId, len: u32) {
    let n1 = self.node_mut(n1_id);
    n1.nodes.push(NodeLink {
      node: n2_id,
      len
    });

    let n2 = self.node_mut(n2_id);
    n2.nodes.push(NodeLink {
      node: n1_id,
      len
    });
  }

  pub fn node_id_by_osm_id(&self, id: u64) -> Option<NodeId> {
    self.node_map.get(&id).copied()
  }

  fn reset_graph(&mut self) {
    for _ in 0..self.additional_nodes_num {
      let n = self.nodes.pop().unwrap();
      let node_id = NodeId(self.nodes.len());

      for link in n.nodes.iter() {
        let node = self.node_mut(link.node);
        let ind = node.nodes.iter()
          .position(|l| l.node == node_id).unwrap();
        node.nodes.remove(ind);
      }
    }
    self.additional_nodes_num = 0;

    for n in self.nodes.iter_mut() {
      n.eta = u32::MAX;
    }
  }

  pub fn shortest_path(&mut self, start: NodeId, end: NodeId) -> PathResult {
    let mut queue = BinaryHeap::new();
    self.node_mut(start).eta = 0;
    let start_node = self.node(start);
    let end_node = self.node(end);

    queue.push(State {
      cost: start_node.eta + distance_t(start_node, end_node, Kmh(50)),
      node: start
    });

    while let Some(state) = queue.pop() {
      if state.node == end {
        debug!("queue len = {}", queue.len());
        debug!("dist = {}", self.node(state.node).eta);
        break;
      }

      let node = self.node(state.node);

      match node.kind {
        NodeKind::Plain => {
          for link in node.nodes.iter() {
            let next_node = self.node_mut(link.node);

            match next_node.kind {
              NodeKind::Plain => {
                if next_node.eta > node.eta + link.len {
                  next_node.eta = node.eta + link.len;
                  let dist = distance_t(next_node, end_node, Kmh(50));
                  queue.push(State { cost: next_node.eta + dist, node: link.node });
                }
              },

              NodeKind::Car { eta, .. } => {
                if node.eta as i64 <= eta {
                  let link_len = link.len + (eta - node.eta as i64) as u32;

                  if next_node.eta > node.eta + link_len {
                    next_node.eta = node.eta + link_len;
                    let dist = distance_t(next_node, end_node, Kmh(50));
                    queue.push(State { cost: next_node.eta + dist, node: link.node });
                  }
                }
              }
            }
          }
        },

        NodeKind::Car {..} => {
          for link in node.nodes.iter() {
            let next_node = self.node_mut(link.node);

            if next_node.eta > node.eta + link.len {
              next_node.eta = node.eta + link.len;
              let dist = distance_t(next_node, end_node, Kmh(50));
              queue.push(State { cost: next_node.eta + dist, node: link.node });
            }
          }
        }
      }
    }

    // assemble path from graph
    let path_result = if end_node.eta == u32::MAX {
      PathResult::default()
    } else {
      let mut path = vec![MapPoint::from(self.node(end))];
      let mut path_etas = vec![self.node(end).eta];

      let mut curr_node = end_node;
      'main: while curr_node.eta != 0 {
        trace!("\n---------------curr_node----------------");
        trace!("id: {} kind: {:?} eta: {}\n----", curr_node.id, curr_node.kind, curr_node.eta);

        match curr_node.kind {
          NodeKind::Plain => {
            for link in curr_node.nodes.iter() {
              let n = self.node(link.node);

              if n.eta < curr_node.eta && n.eta == curr_node.eta.overflowing_sub(link.len).0 {
                trace!("id: {} kind: {:?} eta: {} = {} link_len: {}", n.id, n.kind, n.eta, curr_node.eta.overflowing_sub(link.len).0, link.len);

                curr_node = n;
                path.push(MapPoint::from(n));
                path_etas.push(n.eta);

                continue 'main;
              }
            }
          },
          NodeKind::Car {eta, ..} => {
            for link in curr_node.nodes.iter() {
              let n = self.node(link.node);
              if n.eta < curr_node.eta {
                match n.kind {
                  NodeKind::Plain => {
                    if n.eta == curr_node.eta.overflowing_sub(link.len + (eta - n.eta as i64) as u32).0 && path[path.len() - 2].id != n.id {
                      trace!("id: {} kind: {:?} eta: {} = {} link_len: {}", n.id, n.kind, n.eta, curr_node.eta.overflowing_sub(link.len + (eta - n.eta as i64) as u32).0, link.len);

                      curr_node = n;
                      path.push(MapPoint::from(n));
                      path_etas.push(n.eta);

                      continue 'main;
                    }
                  },
                  NodeKind::Car { .. } => {
                    if n.eta == curr_node.eta.overflowing_sub(link.len).0 {
                      trace!("id: {} kind: {:?} eta: {} = {} link_len: {}", n.id, n.kind, n.eta, curr_node.eta.overflowing_sub(link.len + ROAD_TO_CAR).0, link.len);

                      curr_node = n;
                      path.push(MapPoint::from(n));
                      path_etas.push(n.eta);

                      continue 'main;
                    }
                  }
                }
              }
            }
          }
        }
        error!("Couldn't find path");
        break;
      }

      path.reverse();
      path_etas.reverse();

      PathResult {
        total_time: *path_etas.last().unwrap(),
        points: path,
        eta_list: path_etas
      }
    };

    self.reset_graph();

    path_result
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
  pub id: u64,
  pub lon: f64,
  pub lat: f64
}

#[derive(Debug, Serialize)]
pub enum NodeKind {
  Plain,
  Car {
    eta: i64,
    free_seats: u8
  }
}
#[derive(Serialize)]
pub struct NodeLink {
  node: NodeId,
  len: u32,
}