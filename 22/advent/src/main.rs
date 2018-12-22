extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

const DEPTH: usize = 11739;
const TX: usize = 11;
const TY: usize = 718;

const NY: usize = 800;
const NX: usize = 40;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
enum Type {
    Rocky,
    Wet,
    Narrow,
}
use Type::*;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
enum Equip {
    Torch,
    Climbing,
    Neither,
}
use Equip::*;

fn can_use(region: Type, equip: Equip) -> bool {
    match (region, equip) {
        (Rocky, Climbing) | (Rocky, Torch) => true,
        (Rocky, _) => false,
        (Wet, Climbing) | (Wet, Neither) => true,
        (Wet, _) => false,
        (Narrow, Torch) | (Narrow, Neither) => true,
        (Narrow, _) => false,
    }
}

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
    let mut cave = [[0usize; NX]; NY];
    for y in 0..NY {
        for x in 0..NX {
            if let Some(idx) = geoindex(x, y) {
                cave[y][x] = erosion_level(idx);
            } else {
                cave[y][x] = erosion_level(cave[y - 1][x] * cave[y][x - 1])
            }
        }
    }
    let mut real = [[Rocky; NX]; NY];
    for y in 0..NY {
        for x in 0..NX {
            real[y][x] = type_from_level(cave[y][x]);
        }
    }

    let mut result = 0;
    for y in 0..=TY {
        for x in 0..=TX {
            result += risk(real[y][x]);
        }
    }
    println!("Part 1: {}", result);

    result = find_path(real);
    println!("Part 2: {}", result);
}

fn geoindex(x: usize, y: usize) -> Option<usize> {
    if x == 0 && y == 0 {
        return Some(0);
    }
    if x == TX && y == TY {
        return Some(0);
    }
    if y == 0 {
        return Some(x * 16807);
    }
    if x == 0 {
        return Some(y * 48271);
    }
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

fn risk(typ: Type) -> usize {
    match typ {
        Rocky => 0,
        Wet => 1,
        Narrow => 2,
    }
}

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: (usize, usize, Equip),
}

impl Ord for Equip {
    fn cmp(&self, _other: &Equip) -> Ordering {
        Ordering::Equal
    }
}

impl PartialOrd for Equip {
    fn partial_cmp(&self, _other: &Equip) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // make it a min heap
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_path(grid: [[Type; NX]; NY]) -> usize {
    let start = (0, 0, Torch);
    let goal = (TX, TY, Torch);

    let mut dist = HashMap::new();
    for y in 0..NY {
        for x in 0..NX {
            if can_use(grid[y][x], Torch) {
                dist.insert((x, y, Torch), std::usize::MAX);
            }
            if can_use(grid[y][x], Climbing) {
                dist.insert((x, y, Climbing), std::usize::MAX);
            }
            if can_use(grid[y][x], Neither) {
                dist.insert((x, y, Neither), std::usize::MAX);
            }
        }
    }
    dist.insert(start, 0);

    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        pos: start,
    });

    while let Some(State { cost, pos }) = heap.pop() {
        if pos == goal {
            return cost;
        }

        if cost > *dist.get(&pos).unwrap() {
            continue;
        }

        for n in neigh(pos, &grid) {
            let alt = cost + if pos.2 != n.2 { 7 } else { 1 };
            let next = State { cost: alt, pos: n };
            if next.cost < *dist.get(&next.pos).unwrap() {
                heap.push(next);
                dist.insert(next.pos, next.cost);
            }
        }
    }

    return std::usize::MAX;
}

fn neigh(pos: (usize, usize, Equip), grid: &[[Type; NX]; NY]) -> Vec<(usize, usize, Equip)> {
    let mut result = Vec::with_capacity(7);
    if pos.2 != Torch && can_use(grid[pos.1][pos.0], Torch) {
        result.push((pos.0, pos.1, Torch));
    }
    if pos.2 != Climbing && can_use(grid[pos.1][pos.0], Climbing) {
        result.push((pos.0, pos.1, Climbing));
    }
    if pos.2 != Neither && can_use(grid[pos.1][pos.0], Neither) {
        result.push((pos.0, pos.1, Neither));
    }
    // up
    if pos.1 > 0 {
        if can_use(grid[pos.1 - 1][pos.0], pos.2) {
            result.push((pos.0, pos.1 - 1, pos.2));
        }
    }
    // down
    if pos.1 < NY - 1 {
        if can_use(grid[pos.1 + 1][pos.0], pos.2) {
            result.push((pos.0, pos.1 + 1, pos.2));
        }
    }
    // left
    if pos.0 > 0 {
        if can_use(grid[pos.1][pos.0 - 1], pos.2) {
            result.push((pos.0 - 1, pos.1, pos.2));
        }
    }
    // right
    if pos.0 < NX - 1 {
        if can_use(grid[pos.1][pos.0 + 1], pos.2) {
            result.push((pos.0 + 1, pos.1, pos.2));
        }
    }
    result
}
