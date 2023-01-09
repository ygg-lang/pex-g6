use std::str::FromStr;
use crate::Graph6Error;

#[derive(Copy, Debug, Clone)]
pub struct Graph6 {
    rank: usize,
}

impl FromStr for Graph6 {
    type Err = Graph6Error;

    fn from_str(s: &str) -> Result<Self, Graph6Error> {
        let bytes = remove_head(s.as_bytes());
        println!("{:?}", bytes);
        todo!()
    }
}

fn remove_head(bytes: &[u8]) -> &[u8] {
    if bytes.starts_with(b">>graph6<<") {
        &bytes[10..]
    } else {
        bytes
    }
}