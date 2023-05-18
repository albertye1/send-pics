use std::env;

use crate::args::GenericArgs;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let inst = &args[1]; // which instruction we are dealing with
    println!("{}", inst);
    // is casework really the best way to handle this? 
    if inst == "encode" {
        encode(args::EncodeArgs{args[2], args[3], args[4], args[5]});
    }
}
