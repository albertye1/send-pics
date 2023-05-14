#![allow(unused_variables)]
use std::convert::TryFrom;
use std::fmt::Display;
use std::fmt::Formatter;
// use std::result::Result;
use std::str;
use crc::{Crc, CRC_32_ISO_HDLC};
use crate::chunk_type::ChunkType;
use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct Chunk {
    length: u32,
    ctype: ChunkType,
    data: Vec<u8>,
    crc: u32
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let mut crc_data: Vec<u8> = Vec::new();
        crc_data.append(&mut chunk_type.bytes().to_vec());
        crc_data.append(&mut data.clone());
        Chunk {ctype: chunk_type, data: data.clone(), length: data.len() as u32, crc: crc.checksum( &crc_data)}
    }

    pub fn length(&self) -> u32 {
        self.length as u32
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.ctype
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        let mut string:String = String::new();
        for byte in &self.data {
            string.push(*byte as char);
        }
        Ok(string)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length.to_be_bytes().iter().chain(self.ctype.bytes().iter()).chain(self.data.iter()).chain(self.crc.to_be_bytes().iter()).copied().collect()
    }

}

pub fn u8_arr_to_u32(arr: &[u8]) -> u32 {
    ((arr[0] as u32) << 24) + ((arr[1] as u32) << 16) + ((arr[2] as u32) << 8) + ((arr[3] as u32) << 0)
}
impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let mut chunk_type_arr: [u8; 4] = [0, 0, 0, 0];
        chunk_type_arr.copy_from_slice(&value[4..8],);

        let chunk_type_res = ChunkType::try_from(chunk_type_arr);
        let chunk_type: ChunkType;
        if chunk_type_res.is_err() {
            return Err("Invalid Chunk Type.".into());
        } else {
            chunk_type = chunk_type_res.unwrap();
        }

        let length: u32 = u8_arr_to_u32(&value[0..4]);
        let crc: u32 = u8_arr_to_u32(&value[(value.len() - 4)..value.len()]);
        let mut data: Vec<u8> = Vec::new();

        for data_byte in &value[8..(value.len() - 4)] {
            data.push(*data_byte);
        }

        let chunk: Chunk = Chunk {
            ctype: chunk_type.clone(), data: data.clone(), length: length, crc: crc
        };

        let exp_crc: u32 = Chunk::new(chunk_type.clone(), data.clone()).crc();

        if exp_crc == crc {
            Ok(chunk)
        } else {
            Err("invalid CRC.".into())
        }
    }
}

pub fn vec_to_string(vec: &Vec<u8>) -> String {
    let mut string: String = String::new();
    for byte in vec {
        string.push((*byte) as char);
    }
    string
} 

//need to pass by reference or else ownership changes
impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &vec_to_string(&self.data))
    }
}

// fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}
