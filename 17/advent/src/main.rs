extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

const N: usize = 2000;
const SX: usize = 500;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Tile {
    Empty,
    Clay,
    WaterPour,
    WaterStand,
    Spring,
}
use Tile::*;

fn main() {
    let mut grid = [[Empty; N]; N];
    grid[0][SX] = Spring;
    let mut min_x = std::usize::MAX;
    let mut max_x = std::usize::MIN;
    let mut min_y = std::usize::MAX;
    let mut max_y = std::usize::MIN;
    for line in include_str!("../input.txt").lines() {
        let first_char = line.chars().next().unwrap();
        if first_char == 'x' {
            let parts = line.split(", y=").collect::<Vec<_>>();
            let x = parts[0].split("=").collect::<Vec<_>>()[1]
                .parse::<usize>()
                .unwrap();
            let ys = parts[1].split("..").collect::<Vec<_>>();
            let y_start = ys[0].parse::<usize>().unwrap();
            let y_end = ys[1].parse::<usize>().unwrap();

            for y in y_start..(y_end + 1) {
                grid[y][x] = Clay;
            }
            min_x = std::cmp::min(min_x, x);
            max_x = std::cmp::max(max_x, x);
            min_y = std::cmp::min(min_y, y_start);
            max_y = std::cmp::max(max_y, y_end);
        } else {
            let parts = line.split(", x=").collect::<Vec<_>>();
            let y = parts[0].split("=").collect::<Vec<_>>()[1]
                .parse::<usize>()
                .unwrap();
            let xs = parts[1].split("..").collect::<Vec<_>>();
            let x_start = xs[0].parse::<usize>().unwrap();
            let x_end = xs[1].parse::<usize>().unwrap();

            for x in x_start..(x_end + 1) {
                grid[y][x] = Clay;
            }
            min_x = std::cmp::min(min_x, x_start);
            max_x = std::cmp::max(max_x, x_end);
            min_y = std::cmp::min(min_y, y);
            max_y = std::cmp::max(max_y, y);
        }
    }

    run(&mut grid, min_y, max_y);
    let result = count(&grid, min_y, max_y);
    println!("result: {}", result);

    print(&grid);
}

fn print(grid: &[[Tile; N]; N]) {
    for y in 0..14 {
        for x in 494..508 {
            match grid[y][x] {
                Empty => print!("."),
                Clay => print!("#"),
                WaterPour => print!("|"),
                WaterStand => print!("~"),
                Spring => print!("+"),
            }
        }
        println!("");
    }
    println!("");
}

fn run(grid: &mut [[Tile; N]; N], min: usize, max: usize) {
    let mut start = vec![(1, SX)];

    loop {
        let mut next = vec![];

        let mut did_work = false;
        for points in start {
            let mut y = points.0;
            let mut x = points.1;
            if y > max {
                continue;
            }
            did_work = true;

            while y < N && grid[y][x] != Clay {
                grid[y][x] = WaterPour;
                y += 1;
            }
            if y == N {
                continue;
            }
            loop {
                y -= 1;
                let mut floor = true;
                let mut right = false;
                for xx in x + 1..N {
                    if grid[y][xx] == Clay {
                        right = true;
                        break;
                    }
                    if grid[y + 1][xx] != Clay && grid[y + 1][xx] != WaterStand {
                        floor = false;
                        break;
                    }
                }
                let mut left = false;
                for xx in 0..x {
                    if grid[y][(x - 1) - xx] == Clay {
                        left = true;
                        break;
                    }
                    if grid[y + 1][x - 1 - xx] != Clay && grid[y + 1][x - 1 - xx] != WaterStand {
                        floor = false;
                        break;
                    }
                }
                if right && left {
                    grid[y][x] = WaterStand;
                    for xx in 0..x {
                        if grid[y][(x - 1) - xx] != Clay {
                            grid[y][(x - 1) - xx] = WaterStand;
                        } else {
                            break;
                        }
                    }
                    for xx in x + 1..N {
                        if grid[y][xx] != Clay {
                            grid[y][xx] = WaterStand;
                        } else {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
            let mut left_offset = 1;
            let mut right_offset = 1;
            grid[y][x] = WaterPour;
            loop {
                match (
                    grid[y][x - left_offset],
                    grid[y + 1][x - left_offset],
                    grid[y][x + right_offset],
                    grid[y + 1][x + right_offset],
                ) {
                    (Clay, Clay, Empty, Clay)
                    | (Clay, WaterStand, Empty, Clay)
                    | (Clay, Clay, Empty, WaterStand)
                    | (Clay, Clay, WaterPour, WaterStand)
                    | (Clay, Clay, WaterPour, Clay)
                    | (WaterPour, WaterPour, WaterPour, WaterStand)
                    | (WaterPour, Empty, WaterPour, Clay)
                    | (WaterPour, WaterPour, WaterPour, Clay)
                    | (WaterPour, Empty, WaterPour, WaterStand)
                    | (Empty, Empty, WaterPour, WaterStand)
                    | (Clay, WaterStand, Empty, WaterStand) => {
                        // spread right
                        grid[y][x + right_offset] = WaterPour;
                        right_offset += 1;
                    }
                    (Empty, Clay, Clay, Clay)
                    | (Empty, WaterStand, Clay, Clay)
                    | (Empty, Clay, Clay, WaterStand)
                    | (WaterPour, Clay, Clay, WaterStand)
                    | (WaterPour, WaterStand, Clay, WaterStand)
                    | (WaterPour, WaterStand, WaterPour, Empty)
                    | (WaterPour, Clay, WaterPour, Empty)
                    | (WaterPour, WaterStand, WaterPour, WaterPour)
                    | (WaterPour, Clay, WaterPour, WaterPour)
                    | (Empty, WaterStand, WaterPour, WaterPour)
                    | (Empty, Clay, WaterPour, WaterPour)
                    | (Empty, WaterStand, WaterPour, Empty)
                    | (WaterPour, WaterStand, Clay, Clay)
                    | (WaterPour, WaterStand, Empty, Empty)
                    | (WaterPour, Clay, Clay, Clay)
                    | (Empty, WaterStand, Clay, WaterStand) => {
                        // spread left
                        grid[y][x - left_offset] = WaterPour;
                        left_offset += 1;
                    }
                    (Empty, Clay, Empty, Clay)
                    | (Empty, WaterStand, Empty, Clay)
                    | (Empty, Clay, Empty, WaterStand)
                    | (Empty, WaterStand, WaterPour, Clay)
                    | (WaterPour, Clay, Empty, Clay)
                    | (Empty, WaterStand, WaterPour, WaterStand)
                    | (Empty, Clay, WaterPour, Clay)
                    | (WaterPour, WaterStand, Empty, WaterStand)
                    | (WaterPour, Clay, Empty, WaterStand)
                    | (WaterPour, Clay, WaterPour, WaterStand)
                    | (WaterPour, WaterStand, Empty, Clay)
                    | (Empty, WaterStand, Empty, WaterStand) => {
                        // spread both
                        grid[y][x + right_offset] = WaterPour;
                        right_offset += 1;
                        grid[y][x - left_offset] = WaterPour;
                        left_offset += 1;
                    }
                    (Empty, Empty, Empty, Empty) | (WaterPour, Empty, WaterPour, Empty)
                        | (WaterPour, WaterPour, WaterPour, WaterPour) => {
                        // spill both
                        grid[y][x + right_offset] = WaterPour;
                        grid[y][x - left_offset] = WaterPour;
                        next.push((y + 1, x + right_offset));
                        next.push((y + 1, x - left_offset));
                        break;
                    }
                    (Clay, _, Empty, Empty)
                        | (Clay, Clay, WaterPour, WaterPour)
                        | (Clay, _, WaterPour, Empty) => {
                        // spill right
                        grid[y][x + right_offset] = WaterPour;
                        next.push((y + 1, x + right_offset));
                        break;
                    }
                    (Empty, Empty, Clay, _) | (WaterPour, Empty, Clay, _)
                        | (WaterPour, WaterPour, Clay, Clay) => {
                        // spill left
                        grid[y][x - left_offset] = WaterPour;
                        next.push((y + 1, x - left_offset));
                        break;
                    }
                    (Empty, Empty, Empty, _) => {
                        grid[y][x + right_offset] = WaterPour;
                        right_offset += 1;
                    }
                    (Empty, _, Empty, Empty) => {
                        grid[y][x - left_offset] = WaterPour;
                        left_offset += 1;
                    }
                    (WaterPour, WaterStand, WaterPour, WaterStand)
                    | (WaterPour, WaterStand, WaterPour, Clay)
                    | (WaterPour, Clay, WaterPour, WaterStand)
                    | (WaterPour, Clay, WaterPour, Clay) => {
                        grid[y][x + right_offset] = WaterPour;
                        right_offset += 1;
                        grid[y][x - left_offset] = WaterPour;
                        left_offset += 1;
                    }
                    (a, b, c, d) => panic!(
                        "bad input: ({}, {})({} {}): ({:?}, {:?}, {:?}, {:?})",
                        x, y, left_offset, right_offset, a, b, c, d
                    ),
                }
            }
        }
        next.sort();
        next.dedup();
        start = next;
        if !did_work {
            break;
        }
    }
}

fn count(grid: &[[Tile; N]; N], min: usize, max: usize) -> usize {
    let mut count = 0;
    for x in 0..N {
        for y in min..max + 1 {
            if grid[y][x] == WaterStand || grid[y][x] == WaterPour || grid[y][x] == Spring {
                count += 1;
            }
        }
    }
    count
}
