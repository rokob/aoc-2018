extern crate utils;
use std::collections::{HashSet, HashMap};

fn main() {
    let mut count2 = 0;
    let mut count3 = 0;
    for line in utils::get_lines("advent.txt") {
        let line = line.unwrap();
        let mut letter_count = HashMap::new();
        for c in line.chars() {
            let e = letter_count.entry(c).or_insert(0);
            *e += 1;
        }
        let mut contains_two = false;
        let mut contains_three = false;
        for (k, v) in letter_count.iter() {
            if *v == 2 {
                contains_two = true;
            }
            if *v == 3 {
                contains_three = true;
            }
        }
        if contains_two {
            count2 += 1;
        }
        if contains_three {
            count3 += 1;
        }
    }
    println!("{}", count2 * count3);
}

