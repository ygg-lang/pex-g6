pub type Graph6Result<T> = Result<T, Graph6Error>;


#[derive(Debug, Copy, Clone)]
pub enum Graph6Error {
    InvalidAdjacencyMatrix,
    InvalidDigraphHeader {
        except: char
    },
    GraphTooLarge,
    InvalidSizeChar,
    UnknownError,
    OutOfRange {
        position: usize,
        max: usize,
    },
}

