use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub use std::collections::{HashMap, HashSet};

pub fn get_lines(filename: &str) -> impl Iterator<Item = io::Result<String>> {
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);
    f.lines()
}

pub fn read_file(filename: &str) -> impl Iterator<Item = String> {
    get_lines(filename).map(|s| s.unwrap())
}

pub fn split_ws(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}
