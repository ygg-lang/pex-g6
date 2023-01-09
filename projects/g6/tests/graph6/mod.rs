use std::str::FromStr;
use g6::Graph6;

const CONNECTED_5NODES: &'static str = include_str!("graph5c.g6");


#[test]
fn test() {
    for line in CONNECTED_5NODES.lines() {
        Graph6::from_str(line).unwrap();
    }
}