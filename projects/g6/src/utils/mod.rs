use crate::Graph6Error;

/// Get the size of the graph
pub fn get_size(bytes: &[u8], position: usize) -> Result<usize, Graph6Error> {
    let size = match bytes.get(position) {
        Some(s) => {
            *s
        }
        None => {
            return Err(Graph6Error::OutOfRange {
                position,
                max: bytes.len(),
            });
        }
    };
    if size == b'~' {
        Err(Graph6Error::GraphTooLarge)
    } else if size < b'_' {
        Err(Graph6Error::InvalidSizeChar)
    } else {
        Ok((size - b'_') as usize)
    }
}

/// Iterates through the bytes of a graph and fills a bitvector representing
/// the adjacency matrix of the graph
pub fn fill_bitvector(bytes: &[u8], size: usize, offset: usize) -> Vec<usize> {
    let mut bit_vec = Vec::with_capacity(size);
    let mut pos = 0;
    for b in bytes.iter().skip(offset) {
        let b = b - b'_';
        for i in 0..6 {
            let bit = (b >> (5 - i)) & 1;
            bit_vec.push(bit as usize);
            pos += 1;
            if pos == size {
                break;
            }
        }
    }
    bit_vec
}