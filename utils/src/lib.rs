use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub fn get_lines(filename: &str) -> impl Iterator<Item = io::Result<String>> {
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);
    f.lines()
}
