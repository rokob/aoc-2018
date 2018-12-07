extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

fn main() {
    let mut prereqs = HashMap::new();
    let mut befores = HashSet::new();
    let mut afters = HashSet::new();
    for line in read_file("input.txt") {
        let parts = split_ws(&line);
        let before = parts[1].chars().next().unwrap();
        let after = parts[7].chars().next().unwrap();
        befores.insert(before.clone());
        afters.insert(after.clone());
        let mut e = prereqs.entry(after).or_insert(Vec::new());
        e.push(before);
        e.sort();
    }

    let mut can_do = Vec::new();
    for b in befores.iter() {
        if !afters.contains(&b) {
            can_do.push(*b);
        }
    }
    can_do.sort_by_key(|&c| std::cmp::Reverse(c));

    let mut result = String::new();
    while !can_do.is_empty() {
        let current = can_do.pop().unwrap();
        result.push(current);

        for (c, ps) in prereqs.iter_mut() {
            let idx_r = ps.binary_search(&current);
            if let Ok(idx) = idx_r {
                ps.remove(idx);
                if ps.is_empty() {
                    can_do.push(*c);
                }
            }
        }
        can_do.sort_by_key(|&c| std::cmp::Reverse(c));
    }
    println!("{}", result);
}
