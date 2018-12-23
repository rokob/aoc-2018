extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

fn main() {
    let mut bots = HashMap::new();
    let mut key = (0, 0, 0); 
    let mut best = 0;
    for line in include_str!("../input.txt").lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let pos_part = parts[0];
        let radius = parts[1].split("=").collect::<Vec<_>>()[1].parse::<isize>().unwrap();
        let pos_nums = &pos_part[5..pos_part.len()-2];
        let pos_nums = pos_nums.split(',').map(|n| n.parse::<isize>().unwrap()).collect::<Vec<_>>();
        let pos = (pos_nums[0], pos_nums[1], pos_nums[2]);
        if radius > best {
            best = radius;
            key = pos;
        }
        bots.insert(pos, radius);
    }

    let mut count = 0;
    for (k, _v) in bots.iter() {
        if is_in_range(&key, k, best) {
            count += 1;
        }
    }

    println!("{}", count);
}

fn is_in_range(a: &(isize, isize, isize), b: &(isize, isize, isize), radius: isize) -> bool {
    let distance = (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs();
    distance <= radius
}
