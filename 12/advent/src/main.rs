extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};
use std::collections::VecDeque;

fn main() {
    let mut lines = read_file("input.txt");
    let initial = lines.next().unwrap();
    let initial = split_ws(&initial)[2];
    lines.next();
    let mut state = String::from("...");
    for c in initial.chars() {
        state.push(c);
    }
    state.push_str("...");
    let mut rules = HashMap::new();
    for rule in lines {
        let parts = split_ws(&rule);
        let result = parts[2].chars().next().unwrap();
        rules.insert(parts[0].to_owned(), result);
    }

    let mut last_score = 0;
    let mut offset = 3;
    let mut gen = 0;
    let same_count = 5;
    let mut diffs = VecDeque::with_capacity(same_count + 1);
    'gens: loop {
        let mut next = String::from("...");
        for idx in 2..state.len()-2 {
            match rules.get(&state[idx-2..idx+3]) {
                Some(&c) => {
                    next.push(c)
                },
                _ => panic!("impossible"),
            }
        }
        next.push_str("...");
        offset += 1;
        state = next;
        gen += 1;

        let score = count(&state, offset);
        if gen == 20 {
            println!("Part 1: {}", score)
        }
        let diff = score - last_score;
        diffs.push_back(diff);
        if diffs.len() > same_count {
            let first = diffs.pop_front().unwrap();
            for &v in diffs.iter() {
                if v != first {
                    last_score = score;
                    continue 'gens;
                }
            }
            break;
        }
        last_score = score;
    }
    let result = count(&state, offset);
    let ans = (50_000_000_000 - gen) * (result - last_score) + result;
    println!("Part 2: {}", ans);
}

fn count(state: &str, offset: isize) -> isize {
    let mut result = 0;
    for (i, c) in state.chars().enumerate() {
        if c == '#' {
            result += i as isize - offset;
        }
    }
    result
}
