use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

use crate::{Result};
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
    let mut png = _png.unwrap();
    let _chunk_type = args.chunk_type();
    // replace chunk if already encoded. don't want redundancy
    if png.chunk_by_type(&_chunk_type).is_some() {
        png.remove_chunk(&_chunk_type).unwrap(); // shouldn't error, but panic in case.
    }
    let chunk_type = ChunkType::from_str(&_chunk_type).unwrap();
    if chunk_type.is_critical() {
        return Err("invalid chunk!".into());
    }
    let new_chunk = Chunk::new(chunk_type, args.msg().into());
    png.append_chunk(new_chunk);
    let new_bytes = png.as_bytes();
    let write_res = fs::write(args.output_file(), new_bytes);
    if write_res.is_err() {
        return Err("write failed".into());
    }
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let p = args.path();
    let _bytes = fs::read(p);
    if _bytes.is_err() {
        return Err("invalid png file!".into());
    }
    let _png = Png::try_from(&(_bytes.unwrap()) as &[u8]);
    if _png.is_err() {
        return Err("invalid png file!".into());
    }
    let png = _png.unwrap();
    let chunk_type = args.chunk_type();
    if png.chunk_by_type(&chunk_type).is_none() {
        return Err("chunk type is missing!".into());
    }
    let chunk = png.chunk_by_type(&chunk_type).unwrap();
    println!("Your message is: {}", chunk.data_as_string().unwrap());
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let p = args.path();
    let _bytes = fs::read(p);
    if _bytes.is_err() {
        return Err("invalid png file!".into());
    }
    let _png = Png::try_from(&(_bytes.unwrap()) as &[u8]);
    if _png.is_err() {
        return Err("invalid png file!".into());
    }
    let mut png = _png.unwrap();
    let chunk_type = args.chunk_type();
    if png.chunk_by_type(&chunk_type).is_none() {
        return Err("chunk type is missing!".into());
    }
    if ChunkType::from_str(&chunk_type).unwrap().is_critical() {
        return Err("i'm not removing that".into());
    }
    let chunk = png.remove_chunk(&chunk_type).unwrap();
    println!("Deleted message: {}", chunk.data_as_string().unwrap());
    let write_res = fs::write(args.path(), png.as_bytes());
    if write_res.is_err() {
        return Err("write failed!".into());
    }
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let p = args.path();
    let _bytes = fs::read(p);
    if _bytes.is_err() {
        return Err("invalid png file!".into());
    }
    let _png = Png::try_from(&(_bytes.unwrap()) as &[u8]);
    if _png.is_err() {
        return Err("invalid png file!".into());
    }
    let png = _png.unwrap();
    let chunks = png.chunks();
    println!("Chunk info:");
    for c in chunks {
        println!("{}", c.chunk_type().to_string());
    }
    Ok(())
}
