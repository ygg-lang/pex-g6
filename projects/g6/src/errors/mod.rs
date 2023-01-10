/// Errors that can occur when parsing a graph6 string.
#[derive(Debug, Copy, Clone)]
pub enum Graph6Error {
    /// The given adjacency matrix is invalid.
    InvalidAdjacencyMatrix,
    /// The given header is invalid.
    InvalidHeader {
        /// The expected header.
        except: &'static str,
    },
    /// The given graph is too large.
    GraphTooLarge,
    /// The given graph is too small.
    InvalidSize,
    /// The given graph is too large.
    UnknownError,
    /// The given graph is too large.
    OutOfRange {
        /// The given position.
        position: usize,
        /// The given maximum.
        max: usize,
    },
}

impl From<std::io::Error> for Graph6Error {
    fn from(_: std::io::Error) -> Self {
        Graph6Error::UnknownError
    }
}