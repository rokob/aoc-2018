extern crate utils;
#[allow(unused_imports)]
use utils::{HashSet, HashMap, read_file, split_ws};

const SIZE: usize = 400;

const TIE: usize = 3;
const MAKE_TIE: usize = 2;
const TAG: usize = 1;
const UNSEEN: usize = 0;

fn main() {
    let mut grid = [[0usize; SIZE]; SIZE];
    let mut counter = 4;
    let mut seen = Vec::new();
    println!("read file");
    for line in read_file("advent.txt") {
        let parts = split_ws(&line);
        let x = parts[0].trim_end_matches(',').parse::<usize>().unwrap();
        let y = parts[1].parse::<usize>().unwrap();
        grid[x][y] = counter;
        seen.push((x,y));
        counter += 1;
    }

    println!("begin first loop");
    for x in 0..SIZE {
        for y in 0..SIZE {
            let mut best_dist = SIZE + 1;
            let mut best_val = 0;
            let mut mult = false;
            for (sx, sy) in seen.iter() {
                let dist = (*sx as isize - x as isize).abs() as usize + (*sy as isize - y as isize).abs() as usize;
                if dist == best_dist {
                    mult = true;
                } else if dist < best_dist {
                    best_dist = dist;
                    best_val = grid[*sx][*sy];
                    mult = false;
                }
            }
            if !mult {
                grid[x][y] = best_val;
            }
        }
    }
    println!("end first loop");

    /*
    loop {
        let mut made_change = false;
        let mut this = HashSet::new();
        for (x,y) in seen.iter() {
            made_change = made_change || fill(*x, *y, &mut grid, &mut this);
        }
        for (x,y) in this.iter() {
            if grid[*x][*y] == MAKE_TIE {
                grid[*x][*y] = TIE;
            } else {
                seen.push((*x, *y));
            }
        }
        if !made_change {
            break;
        }
    }
    */
    println!("get infinite");
    let mut infinite = HashSet::new();
    for n in 0..SIZE {
        infinite.insert(grid[n][0]);
        infinite.insert(grid[n][SIZE - 1]);
        infinite.insert(grid[0][n]);
        infinite.insert(grid[SIZE - 1][n]);
    }
    let mut areas = HashMap::new();
    println!("last loop");
    for x in 0..SIZE {
        for y in 0..SIZE {
            if infinite.contains(&grid[x][y]) {
                continue;
            }
            let mut e = areas.entry(&grid[x][y]).or_insert(0);
            *e += 1;
        }
    }
    let result = areas.iter().map(|(k,v)| v).max();
    println!("{}", result.unwrap());
}

fn fill(x: usize, y: usize, grid: &mut [[usize; SIZE]; SIZE], this: &mut HashSet<(usize, usize)>) -> bool {
    let mut made_change = false;
    if x > 0 {
        if grid[x - 1][y] < TIE {
            if !this.insert((x - 1, y)) {
                made_change = true;
                grid[x - 1][y] = MAKE_TIE;
            } else {
                made_change = true;
                grid[x - 1][y] = grid[x][y];
            }
        }
    }
    if x < SIZE - 1 {
        if grid[x + 1][y] < TIE {
            if !this.insert((x + 1, y)) {
                made_change = true;
                grid[x + 1][y] = MAKE_TIE;
            } else {
                made_change = true;
                grid[x + 1][y] = grid[x][y];
            }
        }
    }
    if y > 0 {
        if grid[x][y - 1] < TIE {
            if !this.insert((x, y - 1)) {
                made_change = true;
                grid[x][y - 1] = MAKE_TIE;
            } else {
                made_change = true;
                grid[x][y - 1] = grid[x][y];
            }
        }
    }
    if y < SIZE - 1 {
        if grid[x][y + 1] < TIE {
            if !this.insert((x, y + 1)) {
                made_change = true;
                grid[x][y + 1] = MAKE_TIE;
            } else {
                made_change = true;
                grid[x][y + 1] = grid[x][y];
            }
        }
    }
    made_change
}
