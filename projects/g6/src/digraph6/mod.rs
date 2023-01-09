use crate::Graph6Error;

/// A directed graph represented by digraph6 format.
#[derive(Copy, Clone, Debug)]
pub struct DiGraph6 {
    nodes: usize,
}



/// 必须是 & 开头
fn check_line_header(bytes: &[u8]) -> Result<(), Graph6Error> {
    match bytes {
        [byte] if b'&'.eq(byte) => Ok(()),
        _ => {
            Err(Graph6Error::InvalidHeader { except: "&" })
        }
    }
}

