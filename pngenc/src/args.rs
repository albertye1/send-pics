use std::env;
use std::path::PathBuf;

// arguments for encode function
pub struct EncodeArgs {
    path: PathBuf,
    chunk_type: String,
    msg: String,
    output_file: String
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

impl EncodeArgs {
    pub fn new(path: PathBuf, chunk_type: String, msg: String, output_file: String) -> EncodeArgs {
        EncodeArgs {path, chunk_type, msg, output_file}
    }

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