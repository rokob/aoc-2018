extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

fn main() {
    let mut lines = include_str!("../input.txt").lines();
    let val = lines.next().unwrap();
    process(val);

    // I don't know why this approach is off by one
    // on my puzzle in input, but correct on all the examples
    process_wat(val);
}

const N: usize = 1000;

fn process(val: &str) {
    let mut grid = [[std::u32::MAX; N]; N];
    let origin = (N / 2, N / 2);
    grid[origin.0][origin.1] = 0;

    recur(&val[1..val.len()-1], &mut grid, origin);

    let mut result = 0;
    let mut count = 0;
    for y in 0..N {
        for x in 0..N {
            if grid[y][x] == std::u32::MAX {
                continue;
            }
            if grid[y][x] >= 1000 {
                count += 1;
            }
            result = std::cmp::max(result, grid[y][x]);
        }
    }
    println!("{}", result);
    println!("{}", count);
}

fn next(pos: (usize, usize), c: char) -> (usize, usize) {
    match c {
        'E' => (pos.0, pos.1 + 1),
        'W' => (pos.0, pos.1 - 1),
        'N' => (pos.0 - 1, pos.1),
        'S' => (pos.0 + 1, pos.1),
        _ => panic!("bad input"),
    }
}

fn recur(val: &str, grid: &mut [[u32; N]; N], pos: (usize, usize)) {
    let mut idx = 0;
    let mut pos = pos;
    let mut chars = val.chars();
    loop {
        if let Some(c) = chars.next() {
            if c == '(' { break; }
            let next_pos = next(pos, c);
            grid[next_pos.0][next_pos.1] = std::cmp::min(grid[next_pos.0][next_pos.1], grid[pos.0][pos.1] + 1);
            pos = next_pos;
            idx += 1;
        } else {
            break;
        }
    }

    if idx >= val.len() { return; }
    let mut depth = 1;
    let mut end = idx + 1;
    for c in chars {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {},
        }
        if depth == 0 { break; }
        end += 1;
    }

    let mut curr = &val[idx+1..end];
    loop {
        let mut k = 0;
        depth = 0;
        for c in curr.chars() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                '|' if depth == 0 => break,
                _ => {}
            }
            k += 1;
        }

        recur(&curr[0..k], grid, pos);
        if k == curr.len() {
            break;
        }
        curr = &curr[k+1..];
    }
    if end < val.len() - 1 {
        recur(&val[end+1..], grid, pos);
    }
}

fn recur_wat(val: &str, idx: usize) -> (usize, usize) {
    let mut skip = false;
    let mut skip_until = idx;
    let mut value = 0;
    let mut best = 0;
    let mut last = ' ';
    for (i, c) in val[idx..].chars().enumerate() {
        if skip {
            if idx+i < skip_until {
                continue;
            }
            skip = false;
        }
        match c {
            ')' | '$' => {
                if last == '|' {
                    return (idx+i+1, 0);
                }
                best = std::cmp::max(best, value);
                return (idx+i+1, best);
            }
            '(' | '^' => {
                let (next_idx, inner_val) = recur_wat(val, idx+i+1);
                skip = true;
                skip_until = next_idx;
                value += inner_val;
            }
            '|' => {
                best = std::cmp::max(best, value);
                value = 0;
            }
            _ => {
                value += 1;
            }
        }
        last = c;
    }
    best = std::cmp::max(best, value);
    return (skip_until, best);
}

fn process_wat(val: &str) {
    let (_, value) = recur_wat(val, 0);
    println!("{}", value);
}
