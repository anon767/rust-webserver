use std::fs::File;
use std::io::prelude::*;
use std::io;

pub fn read_file(filename: &str) -> Result<String, io::Error> {
        let f = File::open(filename);
        let mut s = String::new();
        f?.read_to_string(&mut s)?;
        Ok(s)
}
