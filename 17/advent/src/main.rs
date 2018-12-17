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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Action {
    Nothing,
    Spill,
    Pour,
}
use Action::*;

impl Action {
    fn from(cell: Tile, bottom: Tile) -> Self {
        match (cell, bottom) {
            (Clay, _) => Action::Nothing,
            (Empty, Clay) | (Empty, WaterStand) | (WaterPour, Clay) | (WaterPour, WaterStand) => {
                Action::Spill
            }
            (_, Empty) | (_, WaterPour) => Action::Pour,
            _ => panic!("bad"),
        }
    }
}

fn main() {
    let mut grid = [[Empty; N]; N];
    let (min_y, max_y) = get_grid(&mut grid);
    solve(grid, min_y, max_y, /* part 1? */ false);
}

fn get_grid(grid: &mut [[Tile; N]; N]) -> (usize, usize) {
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
    (min_y, max_y)
}

fn solve(mut grid: [[Tile; N]; N], min_y: usize, max_y: usize, part1: bool) {
    run(&mut grid, max_y);
    let result = count(&grid, min_y, max_y, part1);
    println!("Answer: {}", result);
}

#[allow(dead_code)]
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

fn run(grid: &mut [[Tile; N]; N], max: usize) {
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
                let mut right = false;
                for xx in x + 1..N {
                    if grid[y][xx] == Clay {
                        right = true;
                        break;
                    }
                    if grid[y + 1][xx] != Clay && grid[y + 1][xx] != WaterStand {
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
                let left_action =
                    Action::from(grid[y][x - left_offset], grid[y + 1][x - left_offset]);
                let right_action =
                    Action::from(grid[y][x + right_offset], grid[y + 1][x + right_offset]);
                match (left_action, right_action) {
                    (Nothing, Spill) | (Pour, Spill) => {
                        // spill right
                        grid[y][x + right_offset] = WaterPour;
                        right_offset += 1;
                    }
                    (Spill, Nothing) | (Spill, Pour) => {
                        // spill left
                        grid[y][x - left_offset] = WaterPour;
                        left_offset += 1;
                    }
                    (Spill, Spill) => {
                        // spill both
                        grid[y][x + right_offset] = WaterPour;
                        right_offset += 1;
                        grid[y][x - left_offset] = WaterPour;
                        left_offset += 1;
                    }
                    (Pour, Pour) => {
                        // pour both
                        grid[y][x + right_offset] = WaterPour;
                        grid[y][x - left_offset] = WaterPour;
                        next.push((y + 1, x + right_offset));
                        next.push((y + 1, x - left_offset));
                        break;
                    }
                    (Nothing, Pour) => {
                        // pour right
                        grid[y][x + right_offset] = WaterPour;
                        next.push((y + 1, x + right_offset));
                        break;
                    }
                    (Pour, Nothing) => {
                        // pour left
                        grid[y][x - left_offset] = WaterPour;
                        next.push((y + 1, x - left_offset));
                        break;
                    }
                    (Nothing, Nothing) => panic!("bad input"),
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

fn count(grid: &[[Tile; N]; N], min: usize, max: usize, all: bool) -> usize {
    let mut count = 0;
    for x in 0..N {
        for y in min..max + 1 {
            if grid[y][x] == WaterStand {
                count += 1;
            }
            if all && (grid[y][x] == WaterPour || grid[y][x] == Spring) {
                count += 1;
            }
        }
    }
    count
}
