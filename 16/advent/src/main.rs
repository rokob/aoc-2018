extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

fn main() {
    for (i, line) in include_str!("../input.txt").lines().enumerate() {
        let parts = split_ws(&line);
    }
}
