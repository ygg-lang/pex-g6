use g6::Sparse6;
use std::str::FromStr;

const BASIC_GRAPH: &'static str = include_str!("basic.s6");

#[test]
fn test() {
    for line in BASIC_GRAPH.lines() {
        let graph = Sparse6::from_str(line).unwrap();
        println!("{:?}", graph)
    }
}
