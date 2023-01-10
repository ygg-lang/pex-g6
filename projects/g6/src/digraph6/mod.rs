use crate::{
    utils::{fill_bitset, get_size},
    Graph6Error,
};
use fixedbitset::FixedBitSet;
use std::{
    fmt::{Debug, Formatter},
    str::FromStr,
};

/// A directed graph represented by digraph6 format.
#[derive(Clone)]
pub struct DiGraph6 {
    nodes: usize,
    bitset: FixedBitSet,
}

impl Debug for DiGraph6 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("digraph6")
            .field("nodes", &self.nodes())
            .field("edges", &self.edges())
            .field("adjacency", &self.bitset.to_string())
            .finish()
    }
}

impl FromStr for DiGraph6 {
    type Err = Graph6Error;

    fn from_str(s: &str) -> Result<Self, Graph6Error> {
        let bytes = remove_head(s.as_bytes())?;
        let (nodes, bytes) = get_size(bytes)?;
        let bitset = fill_bitset(bytes, nodes * nodes)?;
        Ok(DiGraph6 { nodes, bitset })
    }
}

impl DiGraph6 {
    /// Get the number of nodes in the graph.
    pub fn nodes(&self) -> usize {
        self.nodes
    }
    /// Get the number of edges in the graph.
    pub fn edges(&self) -> usize {
        self.bitset.ones().count()
    }
}

/// 必须是 & 开头
fn remove_head(bytes: &[u8]) -> Result<&[u8], Graph6Error> {
    if bytes.starts_with(b">>digraph6<<&") {
        Ok(&bytes[13..])
    }
    else if bytes.starts_with(b"&") {
        Ok(&bytes[1..])
    }
    else {
        Err(Graph6Error::InvalidHeader { except: "&" })
    }
}
