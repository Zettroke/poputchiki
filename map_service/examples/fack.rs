use map_service::{MapService, MapPoint};
use map_service::osm_map::{OsmNode, InnerNode};
use std::io::Read;
use std::collections::HashMap;
use map_service::graph::RoadGraph;


fn main() {
    let mut ms = MapService {
        nodes: HashMap::new(),
        ways: HashMap::new(),
        graph: RoadGraph::new()
    };
    let st = std::time::Instant::now();
    // ms.load("map_smol.osm.gz".to_string());
    ms.load("Moscow.osm.gz".to_string());
    println!("{}s", (std::time::Instant::now() - st).as_secs_f64());


    println!("sizeof Node: {}", std::mem::size_of::<OsmNode>());
    println!("sizeof InnerNode: {}", std::mem::size_of::<InnerNode>());

    println!("nodes cnt: {}", ms.nodes.len());
    println!("ways cnt: {}", ms.ways.len());

    let path = vec![
        MapPoint::new(0, 55.78313, 37.74253, None).unwrap(),
        MapPoint::new(0, 55.80125, 37.54111, None).unwrap()
    ];
    ms.build_path_rust(path.iter().collect());

    let avg_way_len = ms.ways.values().map(|v|v.nodes.len()).sum::<usize>() as f64 / ms.ways.len() as f64;
    println!("avg_way_len: {}", avg_way_len);
    std::io::stdin().read(&mut [0u8; 1]).unwrap();
}