use std::env;
use std::path::PathBuf;
use std::process;

use crate::args::{EncodeArgs, DecodeArgs, RemoveArgs, PrintArgs};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn help() {
    println!("Possible Commands:");
    println!("- pngenc encode <filepath> <chunk type> <message> [output file]");
    println!("- pngenc decode <filepath> <chunk type>");
    println!("- pngenc remove <filepath> <chunk type>");
    println!("- pngenc print <filepath>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // clearly they don't know the command!
    if args.len() == 1 {
        help();
        process::exit(0);
    }
    let inst: String = args[1].clone(); // which instruction we are dealing with
    // println!("{}", inst);
    let res: Result<()>;
    // is casework really the best way to handle this? 
    if inst == "help" {
        help();
        res = Ok(());
    } else if inst == "encode" {
        // copy objects, so we don't mover from vector
        let path = PathBuf::from(&args[2]);
        let chunk_type: String = args[3].clone();
        let msg: String = args[4].clone();
        let mut output: PathBuf = path.clone();
        if args.len() > 5 { output = PathBuf::from(&args[5]); }
        // initialize EncodeArgs
        let args: EncodeArgs;
        args = EncodeArgs::new(path, chunk_type, msg, output);
        res = commands::encode(args);
    } else if inst == "decode" {
        // same thing as above, but 
        let path = PathBuf::from(&args[2]);
        let chunk_type = args[3].clone();
        let args: DecodeArgs;
        args = DecodeArgs::new(path, chunk_type);
        res = commands::decode(args);
    } else if inst == "remove" {
        let path = PathBuf::from(&args[2]);
        let chunk_type = args[3].clone();
        let args: RemoveArgs;
        args = RemoveArgs::new(path, chunk_type);
        res = commands::remove(args);
    } else if inst == "print" {
        let path = PathBuf::from(&args[2]);
        let args: PrintArgs; 
        args = PrintArgs::new(path);
        res = commands::print_chunks(args);
    } else {
        panic!("instructions are encode, decode, remove, and print. not whatever you just did");
    }
    assert!(res.is_ok(), "Failed to {}!", inst);
}
