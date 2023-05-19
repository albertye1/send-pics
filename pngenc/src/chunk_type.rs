#![allow(unused_variables)]
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::{Error, Result};

/// A validated PNG chunk type. See the PNG spec for more details.
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug, Clone)]
pub struct ChunkType {
    bytes:[u8;4]
}

impl ChunkType {
    /// Returns the raw bytes contained in this chunk
    pub fn bytes(&self) -> [u8; 4] {
        return self.bytes;
    }

    /// Returns the property state of the first byte as described in the PNG spec
    pub fn is_critical(&self) -> bool {
        let anc_bit = self.bytes[0] & (1 << 5);
        return anc_bit == 0;
    }

    /// Returns the property state of the second byte as described in the PNG spec
    pub fn is_public(&self) -> bool {
        let priv_bit = self.bytes[1] & (1 << 5);
        return priv_bit == 0;
    }

    /// Returns the property state of the third byte as described in the PNG spec
    pub fn is_reserved_bit_valid(&self) -> bool {
        let res_bit = self.bytes[2] & (1 << 5);
        return res_bit == 0;
    }

    /// Returns the property state of the fourth byte as described in the PNG spec
    pub fn is_safe_to_copy(&self) -> bool {
        let safe_bit = self.bytes[3] & (1 << 5);
        return safe_bit >= 1;
    }

    /// Returns true if the reserved byte is valid and all four bytes are represented by the characters A-Z or a-z.
    /// Note that this chunk type should always be valid as it is validated during construction.
    pub fn is_valid(&self) -> bool {
        for byte in self.bytes {
            if !ChunkType::is_valid_byte(byte) {
                return false;
            }
        }
        if self.bytes[2] & (1 << 5) != 0 {
            return false;
        }
        return true;
    }

    /// Valid bytes are represented by the characters A-Z or a-z
    pub fn is_valid_byte(byte: u8) -> bool {
        return (byte >= ('A' as u8) && byte <= ('Z' as u8)) || (byte >= ('a' as u8) && byte <= ('z' as u8));
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        let ex = ChunkType {
            bytes: bytes
        };
        for byte in bytes {
            if !ChunkType::is_valid_byte(byte) {
                return Err("Invalid Byte Array!".into())
            }
        }
        Ok(ex)
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}", self.bytes[0] as char, self.bytes[1] as char, self.bytes[2] as char, self.bytes[3] as char)
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut arr: [u8; 4] = [0; 4];
        for i in 0..4 {
            arr[i] = s.as_bytes()[i];
        }

        let res = ChunkType::try_from(arr);
        res
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &ChunkType) -> bool {
        self.bytes[0] == other.bytes[0] && self.bytes[1] == other.bytes[1] && self.bytes[2] == other.bytes[2] && self.bytes[3] == other.bytes[3]
    }
}

impl Eq for ChunkType {
    /*
    fn eq(&self, other: &ChunkType) -> bool {
        return (self.bytes[0] == other.bytes[0] && self.bytes[1] == other.bytes[1] && self.bytes[2] == other.bytes[2] && self.bytes[3] == other.bytes[3])
    }
    */
}