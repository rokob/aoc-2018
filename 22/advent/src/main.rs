extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

const DEPTH: usize = 11739;
const TX: usize = 11;
const TY: usize = 718;

const N: usize = 720;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
enum Type {
    Rocky,
    Wet,
    Narrow,
}
use Type::*;

use std::fmt;
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Wet => write!(f, "="),
            Rocky => write!(f, "."),
            Narrow => write!(f, "|"),
        }
    }
}

fn main() {
    let mut cave = [[0usize; N]; N];
    for y in 0..N {
        for x in 0..N {
            if let Some(idx) = geoindex(x, y) {
                cave[y][x] = erosion_level(idx);
            } else {
                cave[y][x] = erosion_level(cave[y-1][x] * cave[y][x-1])
            }
        }
    }
    let mut real = [[Rocky; N]; N];
    for y in 0..N {
        for x in 0..N {
            real[y][x] = type_from_level(cave[y][x]);
        }
    }

    let mut result = 0;
    for y in 0..=TY {
        for x in 0..=TX {
            /*
            if x == 0 && y == 0 { print!("M") }
            else if x == TX && y == TX { print!("T") }
            else {
                print!("{}", real[y][x]);
            }
            */
            result += risk(real[y][x]);
        }
        //println!("");
    }
    //println!("");
    println!("{}", result);
}

fn geoindex(x: usize, y: usize) -> Option<usize> {
    if x == 0 && y == 0 { return Some(0); }
    if x == TX && y == TY { return Some(0); }
    if y == 0 { return Some(x * 16807); }
    if x == 0 { return Some(y * 48271); }
    None
}

fn erosion_level(idx: usize) -> usize {
    (idx + DEPTH) % 20183
}

fn type_from_level(level: usize) -> Type {
    match level % 3 {
        0 => Rocky,
        1 => Wet,
        2 => Narrow,
        _ => panic!("bad input"),
    }
}

// 0 for rocky regions, 1 for wet regions, and 2 for narrow regions.
fn risk(typ: Type) -> usize {
    match typ {
        Rocky => 0,
        Wet => 1,
        Narrow => 2,
    }
}

/*
The region at 0,0 (the mouth of the cave) has a geologic index of 0.
The region at the coordinates of the target has a geologic index of 0.
If the region's Y coordinate is 0, the geologic index is its X coordinate times 16807.
If the region's X coordinate is 0, the geologic index is its Y coordinate times 48271.
Otherwise, the region's geologic index is the result of multiplying the erosion levels of the regions at X-1,Y and X,Y-1.
A region's erosion level is its geologic index plus the cave system's depth, all modulo 20183. Then:

If the erosion level modulo 3 is 0, the region's type is rocky.
If the erosion level modulo 3 is 1, the region's type is wet.
If the erosion level modulo 3 is 2, the region's type is narrow.
*/

