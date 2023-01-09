use std::str::FromStr;
use g6::Graph6;


const CONNECTED_5NODES: &'static str = include_str!("graph5c.g6");
const CONNECTED_5EDGES: &'static str = include_str!("ge5c.g6");

#[test]
fn test() {
    println!("{}", 0b00111111 as u8);
    for line in CONNECTED_5NODES.lines() {
        let graph = Graph6::from_str(line).unwrap();
        println!("{:?}", graph)
    }
}