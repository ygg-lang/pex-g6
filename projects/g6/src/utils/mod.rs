#![allow(dead_code, unused)]

use crate::Graph6Error;
use fixedbitset::FixedBitSet;
use std::io::Write;

/// Write the size of the graph
pub fn write_size<W: Write>(buffer: &mut W, size: usize) -> Result<usize, Graph6Error> {
    if size < 63 {
        buffer.write(&[(size + 63) as u8])?;
        Ok(1)
    }
    else if size < 258048 {
        let x1 = 63 + (size >> 12) as u8;
        let x2 = 63 + (size >> 06) as u8;
        let x3 = 63 + (size >> 00) as u8;
        buffer.write(&[126, x1, x2, x3])?;
        Ok(4)
    }
    else if size < 68719476736 {
        let x1 = 63 + (size >> 30) as u8;
        let x2 = 63 + (size >> 24) as u8;
        let x3 = 63 + (size >> 18) as u8;
        let x4 = 63 + (size >> 12) as u8;
        let x5 = 63 + (size >> 06) as u8;
        let x6 = 63 + (size >> 00) as u8;
        buffer.write(&[126, 126, x1, x2, x3, x4, x5, x6])?;
        Ok(8)
    }
    else {
        Err(Graph6Error::GraphTooLarge)
    }
}

/// Get the size of the graph
pub fn get_size(bytes: &[u8]) -> Result<(usize, &[u8]), Graph6Error> {
    match bytes {
        [b'~', b'~', x1, x2, x3, x4, x5, x6, rest @ ..] => {
            let x1 = (*x1 - b'?') as usize;
            let x2 = (*x2 - b'?') as usize;
            let x3 = (*x3 - b'?') as usize;
            let x4 = (*x4 - b'?') as usize;
            let x5 = (*x5 - b'?') as usize;
            let x6 = (*x6 - b'?') as usize;
            let size = x1 << 30 | x2 << 24 | x3 << 18 | x4 << 12 | x5 << 6 | x6;
            Ok((size, rest))
        }
        // (x1 - 63) * 2^12 + (x2 - 63) * 2^6 + (x3 - 63)
        [b'~', x1, x2, x3, rest @ ..] => {
            let x1 = (*x1 - b'?') as usize;
            let x2 = (*x2 - b'?') as usize;
            let x3 = (*x3 - b'?') as usize;
            let size = x1 << 12 | x2 << 6 | x3;
            Ok((size, rest))
        }
        [x0, rest @ ..] => {
            let x0 = (*x0 - b'?') as usize;
            Ok((x0, rest))
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
