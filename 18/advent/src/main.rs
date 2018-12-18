extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Open,
    Tree,
    Lumber,
}
use Tile::*;

const N: usize = 50;

fn main() {
    let mut area = [[Tile::Open; N]; N];

    for (r, line) in include_str!("../input.txt").lines().enumerate() {
        for (c, val) in line.chars().enumerate() {
            match val {
                '.' => area[r][c] = Open,
                '#' => area[r][c] = Lumber,
                '|' => area[r][c] = Tree,
                _ => panic!("bad input"),
            }
        }
    }

    for _ in 0..10 {
        area = once(area);
    }
    let result = compute(area);

    println!("Part 1: {}", result);
}

fn adj_counts(area: &[[Tile; N]; N], r: usize, c: usize) -> (usize, usize, usize) {
    let mut e = 0;
    let mut t = 0;
    let mut l = 0;
    for rr in 0..3 {
        for cc in 0..3 {
            let rrr = rr as isize - 1;
            let ccc = cc as isize - 1;
            if rrr == 0 && ccc == 0 { continue; }
            let y = r as isize + rrr;
            let x = c as isize + ccc;
            if y < 0 || x < 0 || y >= N as isize  || x >= N as isize { continue; }
            match area[y as usize][x as usize] {
                Open => e += 1,
                Tree => t += 1,
                Lumber => l += 1,
            }
        }
    }
    (e, t, l)
}
/*
 *
In particular:

An open acre will become filled with trees if three or more adjacent acres contained trees. Otherwise, nothing happens.
An acre filled with trees will become a lumberyard if three or more adjacent acres were lumberyards. Otherwise, nothing happens.
An acre containing a lumberyard will remain a lumberyard if it was adjacent to at least one other lumberyard and at least one acre containing trees. Otherwise, it becomes open.
*/

fn once(area: [[Tile; N]; N]) -> [[Tile; N]; N] {
    let mut result = [[Open; N]; N];
    for r in 0..N {
        for c in 0..N {
            let (e, t, l) = adj_counts(&area, r, c);
            match area[r][c] {
                Open => {
                    if t >= 3 {
                        result[r][c] = Tree;
                    }
                },
                Lumber => {
                    if l >= 1 && t >= 1 {
                        result[r][c] = Lumber;
                    } else {
                        result[r][c] = Open;
                    }
                },
                Tree => {
                    if l >= 3 {
                        result[r][c] = Lumber;
                    } else {
                        result[r][c] = Tree;
                    }
                },
            }
        }
    }
    result
}

fn compute(area: [[Tile; N]; N]) -> usize {
    let mut wood = 0;
    let mut lumber = 0;
    for r in 0..N {
        for c in 0..N {
            match area[r][c] {
                Open => {},
                Lumber => lumber += 1,
                Tree => wood += 1,
            }
        }
    }
    wood * lumber
}
