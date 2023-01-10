use std::str::FromStr;
use g6::{DiGraph6};

const CONNECTED_3NODES: &'static str = include_str!("digl3.d6");

#[test]
fn test() {
    for line in CONNECTED_3NODES.lines() {
        let graph = DiGraph6::from_str(line).unwrap();
        println!("{:?}", graph)
    }
}