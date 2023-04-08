#![allow(unused_variables)]
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::{Error, Result};

/// A validated PNG chunk type. See the PNG spec for more details.
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug, Clone, PartialEq, Eq)]

pub struct Chunk {
    ChunkType ctype;
    u8 length;
    str data;
    u8 crc;
}

impl TryFrom<[u8]> for Chunk {
    fn try_from(value: [u8]) -> Result<ChunkType, ChunkType::Error> {

    }
}

impl Error for Chunk {
    
}