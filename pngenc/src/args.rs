use std::env;
use std::path::PathBuf;

// generic enum encompassing all argument types
pub enum GenericArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

// arguments for encode function
pub struct EncodeArgs {
    path: PathBuf,
    chunk_type: String,
    msg: String,
    output_file: String
}

impl EncodeArgs {
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn chunk_type(&self) -> String {
        self.chunk_type.clone()
    }

    pub fn msg(&self) -> String {
        self.msg.clone()
    }

    pub fn output_file(&self) -> String {
        self.output_file.clone()
    }
}

pub struct DecodeArgs {
    path: PathBuf,
    chunk_type: String
}

pub struct RemoveArgs {
    path: PathBuf,
    chunk_type: String
}

pub struct PrintArgs {
    path: PathBuf
}
