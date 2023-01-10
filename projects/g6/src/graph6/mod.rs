use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use fixedbitset::FixedBitSet;
use crate::Graph6Error;
use crate::utils::{fill_bitset, get_size};
mod to_wolfram;

/// A graph represented by graph6 format.
#[derive(Clone)]
pub struct Graph6 {
    nodes: usize,
    bitset: FixedBitSet,
}

impl Debug for Graph6 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("graph6")
            .field("nodes", &self.nodes())
            .field("edges", &self.edges())
            .field("adjacency", &self.bitset.to_string())
            .finish()
    }
}

impl Graph6 {
    /// Get the number of nodes in the graph.
    pub fn nodes(&self) -> usize {
        self.nodes
    }
    /// Get the number of edges in the graph.
    pub fn edges(&self) -> usize {
        self.bitset.ones().count()
    }
}

impl FromStr for Graph6 {
    type Err = Graph6Error;

    fn from_str(s: &str) -> Result<Self, Graph6Error> {
        let bytes = remove_head(s.as_bytes());
        let (nodes, bytes) = get_size(bytes)?;
        let bitset = fill_bitset(bytes, nodes * (nodes - 1) / 2)?;
        Ok(Graph6 {
            nodes,
            bitset,
        })
    }
}

fn remove_head(bytes: &[u8]) -> &[u8] {
    if bytes.starts_with(b">>graph6<<") {
        &bytes[10..]
    } else {
        bytes
    }
}