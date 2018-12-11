extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};
/*
Find the fuel cell's rack ID, which is its X coordinate plus 10.
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
            grid[y - 1][x - 1] = compute(x, y);
        }
    }

    let mut best = 0;
    let mut best_c = (0, 0, 0);

    let mut sums = [[0; N]; N];

    sums[0][0] = grid[0][0];

    for x in 1..N {
        sums[0][x] = grid[0][x] + sums[0][x - 1];
    }
    for y in 1..N {
        sums[y][0] = grid[y][0] + sums[y - 1][0];
    }
    for y in 1..N {
        for x in 1..N {
            sums[y][x] = grid[y][x] + sums[y - 1][x] + sums[y][x - 1] - sums[y - 1][x - 1];
        }
    }

    for k in 1..=N {
        for r in k - 1..N {
            for c in k - 1..N {
                let total = score(&sums, c, r, k);
                if total > best {
                    best = total;
                    // translate bottom right in zero based coords
                    // to top left in 1 based coords
                    best_c = (c + 2 - k, r + 2 - k, k);
                }
            }
        }
    }
    println!("{},{},{}", best_c.0, best_c.1, best_c.2);
}

fn score(sums: &[[isize; N]; N], x: usize, y: usize, k: usize) -> isize {
    let mut total = sums[y][x];
    if y >= k {
        total -= sums[y - k][x];
    }
    if x >= k {
        total -= sums[y][x - k];
    }
    if x >= k && y >= k {
        total += sums[y - k][x - k];
    }
    total
}

fn compute(x: usize, y: usize) -> isize {
    let rack = x as isize + 10;
    let mut power = rack * y as isize;
    power += S;
    power *= rack;
    power /= 10;
    power /= 10;
    power %= 10;
    power - 5
}
