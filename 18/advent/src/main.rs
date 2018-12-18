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

    let mut seen = HashMap::new();
    let first = compute(&area);
    let mut last_score = first;
    seen.insert(first, 0);
    let mut stop_at = 0;
    for n in 1..1_000_000_000 {
        area = once(area);
        let score = compute(&area);
        if n == 10 {
            println!("Part 1: {}", score);
        }
        last_score = score;
        if seen.contains_key(&score) && n > 1000 {
            stop_at = n;
            break;
        } else {
            seen.insert(score, n);
        }
    }
    let x = seen.get(&last_score).unwrap();
    let diff = stop_at - x;

    let k = (1_000_000_000 - stop_at) % diff;
    for _ in 0..k {
        area = once(area);
    }
    let result = compute(&area);

    println!("Part 2: {}", result);
}

fn adj_counts(area: &[[Tile; N]; N], r: usize, c: usize) -> (usize, usize) {
    let mut t = 0;
    let mut l = 0;
    for rr in -1..2 {
        for cc in -1..2 {
            if rr == 0 && cc == 0 { continue; }
            let y = r as isize + rr;
            let x = c as isize + cc;
            if y < 0 || x < 0 || y >= N as isize  || x >= N as isize { continue; }
            match area[y as usize][x as usize] {
                Open => {},
                Tree => t += 1,
                Lumber => l += 1,
            }
        }
    }
    (t, l)
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
            let (t, l) = adj_counts(&area, r, c);
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

fn compute(area: &[[Tile; N]; N]) -> usize {
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
