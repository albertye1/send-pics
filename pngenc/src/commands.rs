use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

use crate::{Error, Result};
use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::{Chunk, ChunkType, Png};

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let p = args.path();
    let _bytes = fs::read(p);
    if _bytes.is_err() {
        return Err("invalid png file!".into());
    }
    let _png = Png::try_from(&(_bytes.unwrap()) as &[u8]);
    if _png.is_err() {
        return Err("invalid png file!".into());
    }
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    Err("skill issue".into())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    Err("skill issue".into())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    Err("skill issue".into())
}
