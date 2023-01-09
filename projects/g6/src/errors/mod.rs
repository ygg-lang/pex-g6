pub type Graph6Result<T> = Result<T, Graph6Error>;


#[derive(Debug, Copy, Clone)]
pub enum Graph6Error {
    InvalidAdjacencyMatrix,
    InvalidHeader {
        except: &'static str,
    },
    GraphTooLarge,
    InvalidSize,
    UnknownError,
    OutOfRange {
        position: usize,
        max: usize,
    },
}

impl From<std::io::Error> for Graph6Error {
    fn from(_: std::io::Error) -> Self {
        Graph6Error::UnknownError
    }
}