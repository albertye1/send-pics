use std::env;
use std::path::PathBuf;

use crate::args::{EncodeArgs, DecodeArgs, RemoveArgs, PrintArgs};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let inst: String = args[1].clone(); // which instruction we are dealing with
    // println!("{}", inst);
    // is casework really the best way to handle this? 
    if inst == "encode" {
        // copy objects, so we don't mover from vector
        let path = PathBuf::from(&args[2]);
        let chunk_type: String = args[3].clone();
        let msg: String = args[4].clone();
        let mut output: String = "".into();
        if (args.len() > 5) { output = args[5].clone(); }
        // initialize EncodeArgs
        let args: EncodeArgs;
        args = EncodeArgs::new(path, chunk_type, msg, output);
        let encode_res = commands::encode(args);
        assert!(encode_res.is_err(), "failed to encode!");
    } else if inst == "decode" {
        // same thing as above, but 
    }
}
