use crate::{
    utils::{fill_bitset, get_size},
    Graph6Error,
};
use fixedbitset::FixedBitSet;
use std::{
    fmt::{Debug, Formatter},
    str::FromStr,
};

/// Sparse6 format
#[derive(Clone)]
pub struct Sparse6 {
    nodes: usize,
    edges: Vec<(usize, usize)>,
}

impl Debug for Sparse6 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("sparse6")
            .field("nodes", &self.nodes())
            .field("edges", &self.edges)
            // .field("adjacency", &self.bitset.to_string())
            .finish()
    }
}

pub struct Sparse6Edge {
    b: bool,
    x: usize,
}

impl Debug for Sparse6Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", if self.b { "1" } else { "0" }, self.x)
    }
}

pub struct Sparse6Edges {
    bits: FixedBitSet,
    group: usize,
    index: usize,
}

impl Iterator for Sparse6Edges {
    type Item = Sparse6Edge;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.bits.len() {
            return None;
        }
        self.index += 1;
        let head = self.bits[self.index];
        let mut x = 0;
        for i in 0..self.group {
            self.index += 1;
            if self.bits[self.index] {
                x |= 1 << i;
            }
        }
        Some(Sparse6Edge { b: head, x })
    }
}

impl FromStr for Sparse6 {
    type Err = Graph6Error;

    fn from_str(s: &str) -> Result<Self, Graph6Error> {
        let bytes = remove_head(s.as_bytes())?;
        let (nodes, bytes) = get_size(bytes)?;
        let bitset = fill_bitset(bytes, bytes.len() * 8 - 1)?;
        let need_bits = (nodes - 1).next_power_of_two().trailing_zeros();
        println!("bits: {}", bitset);
        let iter = Sparse6Edges { bits: bitset, group: need_bits as usize, index: 0 };
        let edges: Vec<_> = iter.collect();
        println!("edges: {:?}", edges);
        Ok(Sparse6 { nodes, edges: vec![] })
    }
}

fn read_byte(input: &[u8]) -> Result<&[u8], Graph6Error> {
    let head = input.get(0).unwrap();
    println!("head: {} -> {}", head, head - 63);
    todo!()
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
    if bytes.starts_with(b">>sparse6<<:") {
        Ok(&bytes[12..])
    }
    else if bytes.starts_with(b":") {
        Ok(&bytes[1..])
    }
    else {
        Err(Graph6Error::InvalidHeader { except: ":" })
    }
}
