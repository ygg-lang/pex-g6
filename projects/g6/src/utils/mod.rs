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