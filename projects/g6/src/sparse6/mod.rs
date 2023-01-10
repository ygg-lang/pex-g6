use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use crate::Graph6Error;
use crate::utils::get_size;

/// Sparse6 format
pub struct Sparse6 {
    nodes:usize
}

impl Debug for Sparse6 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("sparse6")
            .field("nodes", &self.nodes())
            // .field("edges", &self.edges())
            // .field("adjacency", &self.bitset.to_string())
            .finish()
    }
}

impl FromStr for Sparse6 {
    type Err = Graph6Error;

    fn from_str(s: &str) -> Result<Self, Graph6Error> {
        let bytes = remove_head(s.as_bytes())?;
        let (nodes, bytes) = get_size(bytes)?;
        println!("nodes: {}", nodes);
        println!("bytes: {:?}", bytes);
        todo!()
    }
}

impl Sparse6 {
    /// Get the number of nodes in the graph.
    pub fn nodes(&self) -> usize {
        self.nodes
    }
    // /// Get the number of edges in the graph.
    // pub fn edges(&self) -> usize {
    //     self.bitset.ones().count()
    // }
}

/// 必须是 & 开头
fn remove_head(bytes: &[u8]) -> Result<&[u8], Graph6Error> {
    if bytes.starts_with(b">>sparse6<<&") {
        Ok(&bytes[12..])
    } else if bytes.starts_with(b":") {
        Ok(&bytes[1..])
    } else {
        Err(Graph6Error::InvalidHeader { except: ":" })
    }
}

