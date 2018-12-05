extern crate utils;
#[allow(unused_imports)]
use utils::{HashSet, HashMap, read_file, split_ws};

fn main() {
    let mut polymer = Vec::new();
    let mut last_char = ' ';
    for line in read_file("advent.txt") {
        for c in line.chars() {
            if is_match(last_char, c) {
                polymer.pop();
                last_char = ' ';
                continue;
            }
            polymer.push(c);
            last_char = c;
        }
    }

    let mut no = false;
    while !no {
        no = true;
        last_char = ' ';
        let mut new_polymer = Vec::new();
        for c in polymer.into_iter() {
            if is_match(last_char, c) {
                new_polymer.pop();
                no = false;
                last_char = ' ';
            } else {
                new_polymer.push(c);
                last_char = c;
            }
        }
        polymer = new_polymer;
    }
    println!("{}", polymer.len());
}

fn is_match(a: char, b: char) -> bool {
    if a.is_uppercase() && b.is_uppercase() {
        return false;
    }
    if a.is_uppercase() {
        return b.to_ascii_uppercase() == a;
    }
    if b.is_uppercase() {
        return a.to_ascii_uppercase() == b;
    }
    false
}
