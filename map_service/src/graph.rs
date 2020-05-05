use std::ops::{ Deref, DerefMut };
use std::rc::Rc;
use std::cell::UnsafeCell;
use std::fs::read;
use crate::osm_map::OsmNode;

struct RoadGraph {
    nodes: Vec<Node>,
    /// OSM'овские id нод. Прямое соответствие
    osm_nodes_ids: Vec<u64>
}
impl RoadGraph {
    pub fn get_node(&self, id: NodeId) -> &Node {
        self.nodes.get(id.0).unwrap()
    }
    pub fn get_node_mut(&mut self, id: NodeId) -> &mut Node {
        self.nodes.get_mut(id.0).unwrap()
    }

    pub fn add_node(&mut self, node: Node, osm_id: u64) -> NodeId {
        self.nodes.push(node);
        self.osm_nodes_ids.push(osm_id);
        NodeId(self.nodes.len() - 1)
    }

    pub fn connect_one_way(&mut self, n1_id: NodeId, n2_id: NodeId, len: u32) {
        let n1 = self.get_node_mut(n1_id);
        n1.nodes.push(NodeLink {
            node: n2_id,
            time_len: len
        })
    }
    pub fn connect_two_way(&mut self, n1_id: NodeId, n2_id: NodeId, len: u32) {
        let n1 = self.get_node_mut(n1_id);
        n1.nodes.push(NodeLink {
            node: n2_id,
            time_len: len
        });
        let n2 = self.get_node_mut(n2_id);
        n2.nodes.push(NodeLink {
            node: n1_id,
            time_len: len
        });
    }
}

#[derive(Copy, Clone)]
struct NodeId(usize);

struct Node {
    nodes: Vec<NodeLink>,
    eta: u32,
    kind: NodeKind
}

enum NodeKind {
    Plain,
    Car {
        eta: u32,
        free_seats: u8
    }
}

struct NodeLink {
    node: NodeId,
    time_len: u32,
}