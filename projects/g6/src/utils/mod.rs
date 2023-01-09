use std::io::{Write};
use fixedbitset::FixedBitSet;
use crate::Graph6Error;

///   如果 0 <= n <= 62，则将 N(n) 定义为单字节 n+63。
//   如果 63 <= n <= 258047，定义 N(n) 为四个字节
//       126 R(x)，其中 x 是 n 的 bigendian 18 位二进制形式。
//   如果 258048 <= n <= 68719476735，则将 N(n) 定义为八个字节
//       126 126 R(x)，其中 x 是 n 的 bigendian 36 位二进制形式。
//
//   示例：N(30) = 93
//              N(12345) = N(000011 000000 111001) = 126 66 63 120
//              N(460175067) = N(000000 011011 011011 011011 011011 011011)
//                           = 126 126 63 90 90 90 90 90
pub fn write_size<W: Write>(buffer: &mut W, size: usize) -> Result<usize, Graph6Error> {
    if size < 63 {
        buffer.write(&[(size + 63) as u8])?;
        Ok(1)
    } else if size < 258048 {
        buffer.write(&[126])?;
        todo!()
    } else if size < 68719476736 {
        // buffer.push(126);
        // buffer.push(126);
        todo!()
    } else {
        Err(Graph6Error::GraphTooLarge)
    }
}


/// Get the size of the graph
///
pub fn get_size(bytes: &[u8]) -> Result<(usize, &[u8]), Graph6Error> {
    match bytes {
        [b'~', b'~', x1, x2, x3, x4, x5, x6, rest @ ..] => {
            todo!()
        }
        [b'~', x1, x2, x3, rest @ ..] => {
            todo!()
        }
        [x0, rest @ ..] => {
            let size = *x0 - b'?';
            Ok((size as usize, rest))
        }
        _ => Err(Graph6Error::InvalidSize),
    }
}


pub fn base64_to_base256() {
    todo!()
}


pub fn base256_to_base64(base256: &[u8]) -> Result<Vec<u8>, Graph6Error> {
    let mut base64 = Vec::with_capacity(base256.len() * 4 / 3);
    for byte in base256 {
        let mut byte = *byte;
        for _ in 0..4 {
            base64.push((byte & 0b111111) as u8);
            byte >>= 6;
        }
    }
    Ok(base64)
}

pub fn fill_bitset(base64: &[u8], length: usize) -> Result<FixedBitSet, Graph6Error> {
    let mut bitset = FixedBitSet::with_capacity(length);
    let mut index = 0;
    for byte in base64 {
        let mask = *byte - b'?';
        for i in (0u8..6).rev() {
            if index < length {
                bitset.set(index, mask & (1 << i) != 0);
            }
            index += 1;
        }
    }
    Ok(bitset)
}
