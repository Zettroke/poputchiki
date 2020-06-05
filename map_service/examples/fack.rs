use map_service::{MapService, MapPoint, PlainMapCarPath};
use map_service::osm_map::{OsmNode, InnerNode};
use std::io::{Read, Write};
use std::collections::HashMap;
use map_service::graph::RoadGraph;
use std::fs::File;


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
    MapPoint::new(0, 55.78501988250641, 37.73035526275635, None).unwrap(),
    MapPoint::new(0, 55.7865521887118, 37.6966667175293, None).unwrap()
  ];

  let points =  vec![
    MapPoint::new(2007250243, 55.7911141, 37.7738428, None).unwrap(),
    MapPoint::new(5101685132, 55.7907479, 37.7726437, None).unwrap(),
    MapPoint::new(2007249978, 55.7907223, 37.7725598, None).unwrap(),
    MapPoint::new(606193164, 55.7905887, 37.7721201, None).unwrap(),
    MapPoint::new(2007249612, 55.7904494, 37.7716478, None).unwrap(),
    MapPoint::new(5101642250, 55.7901014, 37.7704671, None).unwrap(),
    MapPoint::new(257598683, 55.790055, 37.770313, None).unwrap(),
    MapPoint::new(257598690, 55.7898953, 37.7697616, None).unwrap(),
    MapPoint::new(1319802211, 55.7894775, 37.7684118, None).unwrap(),
    MapPoint::new(1319802224, 55.7893315, 37.7678923, None).unwrap(),
    MapPoint::new(672529923, 55.789178, 37.7673325, None).unwrap(),
    MapPoint::new(1319802264, 55.7890361, 37.7667292, None).unwrap(),
    MapPoint::new(1319802348, 55.7889043, 37.7661227, None).unwrap(),
    MapPoint::new(1319802337, 55.7887844, 37.7654643, None).unwrap(),
    MapPoint::new(161735038, 55.7886992, 37.764858, None).unwrap(),
    MapPoint::new(601414790, 55.7886279, 37.7642916, None).unwrap(),
    MapPoint::new(1319802164, 55.7885648, 37.7637143, None).unwrap(),
    MapPoint::new(1319802261, 55.7885184, 37.7632795, None).unwrap(),
    MapPoint::new(1319802291, 55.7884758, 37.7627031, None).unwrap(),
    MapPoint::new(60789824, 55.7883922, 37.7622754, None).unwrap(),
    MapPoint::new(2106827859, 55.7883747, 37.7618848, None).unwrap(),
    MapPoint::new(4837243858, 55.7883633, 37.7615883, None).unwrap(),
    MapPoint::new(277692163, 55.7883522, 37.761299, None).unwrap(),
    MapPoint::new(1324648788, 55.7880972, 37.7546006, None).unwrap(),
    MapPoint::new(600488806, 55.7879687, 37.7513854, None).unwrap(),
    MapPoint::new(1458596415, 55.7879467, 37.7506363, None).unwrap(),
    MapPoint::new(600485741, 55.7879276, 37.7501104, None).unwrap(),
    MapPoint::new(5112928711, 55.7879227, 37.749967, None).unwrap(),
    MapPoint::new(68921572, 55.7878855, 37.7488778, None).unwrap(),
    MapPoint::new(68921571, 55.7878562, 37.7478998, None).unwrap(),
    MapPoint::new(1253813799, 55.7878395, 37.7476453, None).unwrap(),
    MapPoint::new(1324648530, 55.7878202, 37.7474464, None).unwrap(),
    MapPoint::new(3148825047, 55.7877605, 37.7471374, None).unwrap(),
    MapPoint::new(1324648665, 55.787729, 37.7470138, None).unwrap(),
    MapPoint::new(332378084, 55.7876954, 37.7468692, None).unwrap(),
    MapPoint::new(1324648727, 55.7876346, 37.7466944, None).unwrap(),
    MapPoint::new(68921568, 55.7875805, 37.7465358, None).unwrap(),
    MapPoint::new(3148825033, 55.7875086, 37.7463277, None).unwrap(),
    MapPoint::new(68921567, 55.7873508, 37.7459117, None).unwrap(),
    MapPoint::new(3148825030, 55.7873032, 37.7457811, None).unwrap(),
    MapPoint::new(68921563, 55.7872486, 37.7455793, None).unwrap(),
    MapPoint::new(5112928709, 55.7871946, 37.7452522, None).unwrap(),
    MapPoint::new(6509128827, 55.7871395, 37.744815, None).unwrap(),
    MapPoint::new(6509128826, 55.7870876, 37.7442089, None).unwrap(),
    MapPoint::new(314969149, 55.7869599, 37.7426214, None).unwrap(),
    MapPoint::new(1708335760, 55.7869467, 37.7423448, None).unwrap(),
    MapPoint::new(617037278, 55.7869079, 37.7411818, None).unwrap(),
    MapPoint::new(1194217118, 55.786903, 37.7410469, None).unwrap(),
    MapPoint::new(1732632704, 55.7868629, 37.7401112, None).unwrap(),
    MapPoint::new(1476331977, 55.7868532, 37.7398845, None).unwrap(),
    MapPoint::new(1476331976, 55.7868296, 37.7393328, None).unwrap(),
    MapPoint::new(6680704399, 55.7868074, 37.7388144, None).unwrap(),
    MapPoint::new(275635581, 55.7868036, 37.7387252, None).unwrap(),
    MapPoint::new(6680704400, 55.7867969, 37.7385383, None).unwrap(),
    MapPoint::new(1476331969, 55.7867425, 37.7370249, None).unwrap(),
    MapPoint::new(1201356648, 55.7867365, 37.7368565, None).unwrap(),
    MapPoint::new(1471958617, 55.7867188, 37.7363632, None).unwrap(),
    MapPoint::new(331273721, 55.7867149, 37.7362532, None).unwrap(),
    MapPoint::new(1472249219, 55.7867111, 37.7361528, None).unwrap(),
    MapPoint::new(1704198028, 55.7867005, 37.7356262, None).unwrap(),
    MapPoint::new(1704198034, 55.7866808, 37.7351125, None).unwrap(),
    MapPoint::new(2006747941, 55.7866751, 37.7349731, None).unwrap(),
    MapPoint::new(2014048898, 55.7866162, 37.7336239, None).unwrap(),
    MapPoint::new(1472253151, 55.7866007, 37.733269, None).unwrap(),
    MapPoint::new(313905540, 55.7865962, 37.7331519, None).unwrap(),
    MapPoint::new(1472249215, 55.78659, 37.7330166, None).unwrap(),
    MapPoint::new(1481344992, 55.7865427, 37.7319838, None).unwrap(),
    MapPoint::new(1472249213, 55.7864655, 37.7302961, None).unwrap(),
    MapPoint::new(313905589, 55.7864599, 37.7302006, None).unwrap(),
    MapPoint::new(5287195269, 55.7864516, 37.7300544, None).unwrap(),
    MapPoint::new(4907791678, 55.7861878, 37.7260176, None).unwrap(),
    MapPoint::new(6680678450, 55.7861391, 37.7252724, None).unwrap(),
    MapPoint::new(4907791621, 55.786125, 37.7250573, None).unwrap(),
    MapPoint::new(4907791662, 55.7861172, 37.724938, None).unwrap(),
    MapPoint::new(83239254, 55.7860808, 37.7243809, None).unwrap(),
    MapPoint::new(5079336807, 55.7860661, 37.7242081, None).unwrap(),
    MapPoint::new(5079336804, 55.7858769, 37.7219861, None).unwrap(),
    MapPoint::new(257061411, 55.7855028, 37.7175928, None).unwrap(),
    MapPoint::new(1883068527, 55.7854842, 37.7173762, None).unwrap(),
    MapPoint::new(4909723375, 55.7854122, 37.7165793, None).unwrap(),
    MapPoint::new(4909723373, 55.7853638, 37.7160444, None).unwrap(),
    MapPoint::new(1838466487, 55.785351, 37.7159028, None).unwrap(),
    MapPoint::new(4909723398, 55.7852768, 37.7150926, None).unwrap(),
    MapPoint::new(257060319, 55.7851966, 37.7142176, None).unwrap(),
    MapPoint::new(4911178552, 55.7850405, 37.7125061, None).unwrap(),
    MapPoint::new(4911194641, 55.7850108, 37.7121608, None).unwrap(),
    MapPoint::new(1015090665, 55.7850012, 37.7120498, None).unwrap(),
    MapPoint::new(249683058, 55.7849637, 37.7116138, None).unwrap(),
    MapPoint::new(1884055506, 55.784932, 37.7112754, None).unwrap(),
    MapPoint::new(160685957, 55.7848222, 37.7101027, None).unwrap(),
    MapPoint::new(4909443131, 55.7848062, 37.7099402, None).unwrap(),
    MapPoint::new(4715392355, 55.7847418, 37.7092955, None).unwrap(),
    MapPoint::new(249682879, 55.7846709, 37.7085836, None).unwrap(),
    MapPoint::new(4715392375, 55.7846292, 37.7081596, None).unwrap(),
    MapPoint::new(4913374977, 55.784606, 37.7079192, None).unwrap(),
    MapPoint::new(4715392377, 55.784603, 37.7078879, None).unwrap(),
    MapPoint::new(4715392374, 55.784583, 37.7076898, None).unwrap(),
    MapPoint::new(253390639, 55.7844995, 37.7067924, None).unwrap(),
    MapPoint::new(6680687644, 55.7844846, 37.7066168, None).unwrap(),
    MapPoint::new(667454862, 55.7844373, 37.7060606, None).unwrap(),
    MapPoint::new(4913375039, 55.7844324, 37.7059713, None).unwrap(),
    MapPoint::new(4913375038, 55.7844302, 37.7058519, None).unwrap(),
    MapPoint::new(249682881, 55.7844323, 37.7057545, None).unwrap(),
    MapPoint::new(6680701495, 55.7845804, 37.7041449, None).unwrap(),
    MapPoint::new(253088177, 55.7845839, 37.7041072, None).unwrap(),
    MapPoint::new(253393227, 55.7846449, 37.7035136, None).unwrap(),
    MapPoint::new(1885691828, 55.7847693, 37.7025494, None).unwrap(),
    MapPoint::new(253390285, 55.7847956, 37.7023573, None).unwrap(),
    MapPoint::new(4714015430, 55.7846919, 37.7021981, None).unwrap(),
    MapPoint::new(253391976, 55.7845808, 37.7020118, None).unwrap(),
    MapPoint::new(1885691820, 55.7844212, 37.7017247, None).unwrap(),
    MapPoint::new(1885691822, 55.7844712, 37.7016363, None).unwrap(),
    MapPoint::new(1885691816, 55.7839541, 37.7007314, None).unwrap(),
    MapPoint::new(4714015443, 55.7839027, 37.7006729, None).unwrap(),
    MapPoint::new(1885691810, 55.7838508, 37.7006436, None).unwrap(),
    MapPoint::new(1275897599, 55.7838515, 37.7006141, None).unwrap(),
    MapPoint::new(3499937492, 55.7838719, 37.7001429, None).unwrap(),
    MapPoint::new(3499939993, 55.7838759, 37.7000624, None).unwrap(),
    MapPoint::new(3499939996, 55.7839564, 37.7000556, None).unwrap(),
    MapPoint::new(1275897570, 55.7839692, 37.6995674, None).unwrap(),
    MapPoint::new(1275897603, 55.7839658, 37.699417, None).unwrap(),
    MapPoint::new(1275897585, 55.7839632, 37.6993491, None).unwrap(),
    MapPoint::new(3499940003, 55.784059, 37.6993323, None).unwrap(),
    MapPoint::new(3499940005, 55.7841294, 37.69932, None).unwrap(),
    MapPoint::new(696793498, 55.7847075, 37.6992188, None).unwrap(),
    MapPoint::new(1376525044, 55.7863914, 37.6988644, None).unwrap(),
    MapPoint::new(975727396, 55.7867225, 37.6988845, None).unwrap(),
    MapPoint::new(696793501, 55.7868952, 37.698938, None).unwrap(),
    MapPoint::new(696793502, 55.7880893, 37.6992917, None).unwrap(),
    MapPoint::new(5859686683, 55.7884503, 37.6993758, None).unwrap(),
    MapPoint::new(696793503, 55.7887034, 37.699566, None).unwrap(),
    MapPoint::new(5859686684, 55.7889239, 37.6997627, None).unwrap(),
    MapPoint::new(696793504, 55.7891239, 37.7, None).unwrap(),
    MapPoint::new(1880662031, 55.789244, 37.7001682, None).unwrap(),
    MapPoint::new(1880662045, 55.7893776, 37.7003807, None).unwrap(),
    MapPoint::new(696793505, 55.7894955, 37.7005768, None).unwrap(),
    MapPoint::new(1880663591, 55.7896087, 37.7008001, None).unwrap(),
    MapPoint::new(696793506, 55.7900027, 37.7015906, None).unwrap(),
    MapPoint::new(1880676698, 55.7903503, 37.7022992, None).unwrap(),
    MapPoint::new(1880676954, 55.7907061, 37.7029986, None).unwrap(),
    MapPoint::new(1880677906, 55.7912139, 37.7039377, None).unwrap(),
    MapPoint::new(1880679269, 55.7913943, 37.7042841, None).unwrap(),
    MapPoint::new(696793507, 55.7915675, 37.7046122, None).unwrap(),
    MapPoint::new(1880684238, 55.7917228, 37.704883, None).unwrap(),
    MapPoint::new(696793519, 55.7918444, 37.7050583, None).unwrap(),
    MapPoint::new(1880687192, 55.7919863, 37.7052286, None).unwrap(),
    MapPoint::new(696793520, 55.7921274, 37.7053858, None).unwrap(),
    MapPoint::new(1880693659, 55.79229, 37.7055177, None).unwrap(),
    MapPoint::new(1880694653, 55.7924339, 37.7056038, None).unwrap(),
    MapPoint::new(696793521, 55.7926169, 37.7056778, None).unwrap(),
    MapPoint::new(696793522, 55.7927903, 37.7057102, None).unwrap(),
    MapPoint::new(1880705783, 55.7930038, 37.7057193, None).unwrap(),
    MapPoint::new(1880708084, 55.7931901, 37.7056778, None).unwrap(),
    MapPoint::new(696793523, 55.7933951, 37.7055953, None).unwrap(),
    MapPoint::new(1880712338, 55.7935878, 37.7054684, None).unwrap(),
    MapPoint::new(696793525, 55.7937618, 37.705322, None).unwrap(),
    MapPoint::new(696793526, 55.7940211, 37.7051026, None).unwrap(),
    MapPoint::new(1880719898, 55.7943009, 37.7048347, None).unwrap(),
    MapPoint::new(1880724643, 55.7946289, 37.7044929, None).unwrap(),
    MapPoint::new(696793511, 55.7949018, 37.7041882, None).unwrap(),
    MapPoint::new(4224560978, 55.7951737, 37.7038789, None).unwrap(),
  ];

  let car_path = PlainMapCarPath {
    start_at: 0,
    path: points.iter().collect()
  };

  let res = ms.build_path_using_cars_rust(0, path.iter().collect(), vec![car_path]);
  println!("{:?}", res);
  let s = serde_json::to_string_pretty(&res).unwrap();
  File::create("path.json").unwrap().write_all(s.as_bytes());

  let avg_way_len = ms.ways.values().map(|v|v.nodes.len()).sum::<usize>() as f64 / ms.ways.len() as f64;
  println!("avg_way_len: {}", avg_way_len);
  // std::io::stdin().read(&mut [0u8; 1]).unwrap();
}