use std::convert::TryInto;
use crate::chunk::Chunk;
// signature: 137 80 78 71 13 10 26 10
const STANDARD_HEADER:[u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10]; 

pub struct Png {
    info: Vec<u8>,
    chunks: Vec<Chunk>,
    chunk_end: Vec<i32>
}

impl Png {
    pub fn from_chunk(chunks: Vec<Chunk>) -> Self {
        let mut info = STANDARD_HEADER.to_vec();
        let mut chunk_end: Vec<i32> = Vec::new();
        let chunks_cpy: Vec<Chunk> = chunks.clone();
        for c in chunks_cpy {
            for u in c.data() {
                info.push(*u);
            }
            chunk_end.push(info.len() as i32 - 1);
        }
        print!("{:?}", info);
        Self { info: info, chunks: chunks, chunk_end: chunk_end }
    }

    pub fn append_chunk(&mut self, chunk: Chunk) {
        let _chunk = chunk.clone();
        for u in _chunk.data() {
            self.info.push(*u);
        } 
        self.chunks.push(chunk);
        self.chunk_end.push(self.info.len() as i32 - 1);
    }

    pub fn header(&self) -> &[u8; 8] {
        &STANDARD_HEADER;
    }

    pub fn chunks(&self) -> &[Chunk] {
        
    }
}

