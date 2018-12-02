extern crate utils;
use utils::{HashSet, HashMap, read_file, split_ws};

fn main() {
    for line in read_file("advent.txt") {
        println!("{:?}", split_ws(&line));
    }
}
