use map_service::MapService;
use map_service::osm_map::{OsmNode, InnerNode};
use flate2::bufread::GzDecoder;
use std::io::{BufReader, Read};
use std::fs::File;
use std::collections::HashMap;
use map_service::graph::RoadGraph;


fn main() {
    let mut ms = MapService {
        nodes: Vec::new(),
        ways: HashMap::new(),
        graph: RoadGraph::new()
    };
    let st = std::time::Instant::now();
    ms.load("Moscow.osm".to_string());
    println!("{}s", (std::time::Instant::now() - st).as_secs_f64());


    println!("sizeof Node: {}", std::mem::size_of::<OsmNode>());
    println!("sizeof InnerNode: {}", std::mem::size_of::<InnerNode>());

    println!("nodes cnt: {}", ms.nodes.len());
    println!("ways cnt: {}", ms.ways.len());
    let avg_way_len = ms.ways.values().map(|v|v.nodes.len()).sum::<usize>() as f64 / ms.ways.len() as f64;
    println!("avg_way_len: {}", avg_way_len);
    std::io::stdin().read(&mut [0u8; 1]);
}