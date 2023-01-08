use crate::Graph6Error;

#[derive(Debug)]
pub struct DiGraph {
    pub n: usize,
}



/// 必须是 & 开头
fn check_line_header(bytes: &[u8]) -> Result<(), Graph6Error> {
    match bytes {
        [byte] if b'&'.eq(byte) => Ok(()),
        _ => {
            Err(Graph6Error::InvalidDigraphHeader { except: '&' })
        }
    }
}

