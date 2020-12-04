use std::fs::File;
use std::io::prelude::Read;
use std::io::{BufReader, Result};

pub fn provide_input(path: String) -> Result<String> {
    let file = File::open(path).expect("Error reading file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Error reading contents");

    Ok(contents)
}
