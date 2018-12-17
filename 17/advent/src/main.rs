extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

fn main() {
    for line in include_str!("../input.txt").lines() {
        let parts = split_ws(&line);
    }
}
