extern crate utils;
use std::collections::HashSet;

fn main() {
    let mut sum = 0;
    let mut seen = HashSet::new();
    let mut stream = Vec::new();
    seen.insert(0);
    let mut done = false;
    for line in utils::get_lines("advent.txt") {
        let line = line.unwrap();
        let val = line.parse::<i32>().unwrap();
        sum += val;
        if !seen.insert(sum) {
            println!("Part 2: {}", sum);
            done = true;
            break;
        }
        stream.push(val);
    }
    if !done {
        println!("Part 1: {}", sum);
    } else {
        println!("Part 1 is missing because no loop in part 2");
    }

    while !done {
        for val in &stream {
            sum += *val;
            if !seen.insert(sum) {
                println!("Part 2: {}", sum);
                done = true;
                break;
            }
        }
    }
}
