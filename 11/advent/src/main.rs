extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};
/*
 * Find the fuel cell's rack ID, which is its X coordinate plus 10.
Begin with a power level of the rack ID times the Y coordinate.
Increase the power level by the value of the grid serial number (your puzzle input).
Set the power level to itself multiplied by the rack ID.
Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
Subtract 5 from the power level.
*/

const S: isize = 8561;
const N: usize = 300;

fn main() {
    let mut grid = [[0; N]; N];

    for y in 1..=N {
        for x in 1..=N {
            grid[y-1][x-1] = compute(x, y);
        }
    }

    let mut best = 0;
    let mut best_c = (0, 0, 0);
    for size in 1..=300 {
    for y in 1..=N {
        for x in 1..=N {
            if let Some(v) = score(&grid, x, y, size) {
                if v > best {
                    best = v;
                    best_c = (x, y, size);
                }
            }
        }
    }
    }
    println!("{},{},{}", best_c.0, best_c.1, best_c.2);
}

fn score(grid: &[[isize; N]; N], x: usize, y: usize, size: usize) -> Option<isize> {
    if x + size - 2 < N && y + size - 2 < N {
        let mut result = 0;
        for r in (y-1)..=(y+size-2) {
            for c in (x-1)..=(x+size-2) {
                result += grid[r][c];
            }
        }
        Some(result)
    } else {
        None
    }
}

fn compute(x: usize, y: usize) -> isize {
    let x = x as isize;
    let y = y as isize;

    let rack = x + 10;
    let mut power = rack * y;
    power += S;
    power *= rack;

    for _ in 0..2 {
        power /= 10;
    }

    power = power % 10;

    power - 5
}
