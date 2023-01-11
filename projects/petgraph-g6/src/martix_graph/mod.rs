use g6::{Graph6, Graph6Error};
use petgraph::{graph::UnGraph, matrix_graph::UnMatrix};
use std::str::FromStr;

pub fn import_g6(text: &str) -> Result<UnGraph<u32, ()>, Graph6Error> {
    let graph6 = Graph6::from_str(text)?;
    let mut new_graph = UnGraph::from_edges();
}
