// #![allow(unused_variables)]
// use std::convert::TryFrom;
// use std::fmt;
// use std::str::FromStr;
// use crate::chunk_type::ChunkType;

// use crate::{Error, Result};

// /// A validated PNG chunk type. See the PNG spec for more details.
// /// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
// #[derive(Debug, Clone, PartialEq, Eq)]

// pub struct Chunk {
//     ctype: ChunkType,
//     length: u8,
//     data: str,
//     crc: u8
// }

// impl TryFrom<[u8]> for Chunk {
//     fn try_from(value: [u8]) -> Result<ChunkType> {
        
//     }
// }

// impl std::error::Error for Chunk {
    
// }