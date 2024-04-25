use anyhow::Result;
use std::{fs::File, io::Read};

pub fn get_input(s: &str) -> Result<Box<dyn Read>> {
    let result: Box<dyn Read> = if s == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(s)?)
    };
    Ok(result)
}

pub fn get_content(s: &str) -> Result<Vec<u8>> {
    let mut input = get_input(s)?;
    let mut v = Vec::new();
    input.read_to_end(&mut v).unwrap();
    Ok(v)
}
