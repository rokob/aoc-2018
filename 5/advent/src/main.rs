extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

fn main() {
    let orig = get_polymer();
    let mut best = std::usize::MAX;
    let mut best_c = ' ';
    for w in 0u8..26 {
        let c = ('A' as u8 + w) as char;
        let result = find_length_without(c, &orig);
        if result < best {
            best = result;
            best_c = c;
        }
    }
    println!("{} {}", best_c, best);
}

fn get_polymer() -> Vec<char> {
    read_file("advent.txt").next().unwrap().chars().collect()
}

fn find_length_without(w: char, orig: &Vec<char>) -> usize {
    let mut polymer = orig
        .iter()
        .filter_map(|c| if !same(*c, w) { Some(*c) } else { None })
        .collect::<Vec<_>>();
    let mut no = false;
    while !no {
        no = true;
        let mut last_char = ' ';
        let mut new_polymer = Vec::with_capacity(polymer.len());
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
    polymer.len()
}

fn is_match(a: char, b: char) -> bool {
    let a = a as u8;
    let b = b as u8;
    (a < 91 && b + 65 == a + 97) || (b < 91 && a + 65 == b + 97)
}

fn same(c: char, w: char) -> bool {
    let c = c as u8;
    let w = w as u8;
    c == w || w + 32 == c
}
