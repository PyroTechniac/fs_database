use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::PathBuf;

pub struct Database {
    file: File,
}

impl Database {
    pub fn new(raw: &str) -> Result<Database, &'static str> {
        let path = PathBuf::from(raw);

        if path.is_dir() {
            return Err("Expected a path to a file, got a directory.");
        }

        let file = match path.canonicalize() {
            Ok(f) => OpenOptions::new()
                .read(true)
                .write(true)
                .create(false)
                .open(f)
                .expect("Failed to open file"),
            Err(_) => OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(raw)
                .expect("Failed to create file"),
        };

        Ok(Database { file })
    }
}
