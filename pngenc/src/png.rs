// use std::convert::TryInto;
pub use crate::chunk::Chunk;
use crate::chunk::vec_to_string;
pub use crate::chunk_type::ChunkType;
use std::fmt;
use std::str::FromStr;
use crate::{Result, Error};
// signature: 137 80 78 71 13 10 26 10

pub struct Png {
    chunks: Vec<Chunk>,
}

impl Png {
    const STANDARD_HEADER:[u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10]; 
    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        Self { chunks: chunks }
    }

    pub fn append_chunk(&mut self, chunk: Chunk) {
        let _chunk = chunk.clone();
        self.chunks.push(chunk);
    }

    pub fn remove_chunk(&mut self, chunk_type : &str) -> Result<Chunk> {
        for i in 0..self.chunks.len() {
            let c = self.chunks[i].clone();
            if *c.chunk_type() == ChunkType::from_str(chunk_type).unwrap() {
                self.chunks.remove(i);
                return Ok(c);
            } 
        }
        Err("No such chunk!".into())
    }

    pub fn header(&self) -> &[u8; 8] {
        &Self::STANDARD_HEADER
    }

    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks[..]
    }

    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        for c in &self.chunks {
            if *(c.chunk_type()) == ChunkType::from_str(chunk_type).unwrap() {
                return Some(&c);
            }
        }
        None
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut info = Self::STANDARD_HEADER.to_vec();
        let chunks_cpy: Vec<Chunk> = self.chunks.clone();
        for c in chunks_cpy {
            for u in c.as_bytes() {
                info.push(u);
            }
        }
        info
    }
}

impl TryFrom<&[u8]> for Png { 
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Png> {
        for i in 0..8 {
            if bytes[i] != Png::STANDARD_HEADER[i] {
                return Err("Invalid header".into());
            }
        }
        let mut i = 8;
        let mut chunks: Vec<Chunk> = Vec::new();
        while i < bytes.len() {
            let len = u32::from_be_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]);
            let value = &bytes[i..i + 12 + (len as usize)];
            let c_res = Chunk::try_from(value);
            let chunk;
            if c_res.is_err() {
                return Err("Bad chunk!".into());
            } else {
                chunk = c_res.unwrap();
            }
            chunks.push(chunk);
            i += (12 + len as usize);
        }
        Ok(Png::from_chunks(chunks))
    }
}

impl fmt::Display for Png {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let chunks = self.chunks.clone();
        let mut s = vec_to_string(&chunks[0].data().to_vec());
        for c in &chunks[1..chunks.len()] {
            s = format!("{}, {}", s, vec_to_string(&c.data().to_vec()));
        }
        write!(f, "{}", s)
    }
}