extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

const SIZE: usize = 400;

const TAG: usize = 1;
const UNSEEN: usize = 0;

const DIST: usize = 10000;

fn main() {
    let mut grid = [UNSEEN; SIZE * SIZE];
    let mut counter = TAG + 1;
    let mut seen = Vec::with_capacity(50);
    for line in read_file("advent.txt") {
        let parts = split_ws(&line);
        let x = parts[0].trim_end_matches(',').parse::<usize>().unwrap();
        let y = parts[1].parse::<usize>().unwrap();
        grid[x * SIZE + y] = counter;
        seen.push((x, y));
        counter += 1;
    }

    for x in 0..SIZE {
        for y in 0..SIZE {
            let mut sum = 0;
            for (sx, sy) in seen.iter() {
                let dist = (*sx as isize - x as isize).abs() as usize
                    + (*sy as isize - y as isize).abs() as usize;
                sum += dist;
            }
            if sum <= DIST {
                grid[x * SIZE + y] = TAG;
            }
        }
    }

    let mut area = 0;
    for x in 0..SIZE {
        for y in 0..SIZE {
            if grid[x * SIZE + y] == TAG {
                area += 1;
            }
        }
    }
    println!("{}", area);
}
