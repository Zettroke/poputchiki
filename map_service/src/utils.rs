use std::str::FromStr;

pub fn u64_parse(s: &[u8]) -> u64 {
    u64::from_str(std::str::from_utf8(s).unwrap()).unwrap()
}

pub fn f64_parse(s: &[u8]) -> f64 {
    f64::from_str(std::str::from_utf8(s).unwrap()).unwrap()
}