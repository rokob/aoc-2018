extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};
use std::collections::VecDeque;

const N: usize = 32;

fn main() {
    let result = run("input.txt", false);
    println!("{}", result);
}

#[test]
fn test_fake() {
    assert_eq!(run("fake.txt", false), 18740);
}

#[test]
fn test_fake2() {
    assert_eq!(run("fake2.txt", false), 27730);
}

#[test]
fn test_fake3() {
    assert_eq!(run("fake3.txt", false), 28944);
}

#[test]
fn test_fake4() {
    assert_eq!(run("fake4.txt", false), 39514);
}

#[test]
fn test_fake5() {
    assert_eq!(run("fake5.txt", false), 36334);
}

fn run(filename: &'static str, debug: bool) -> usize {
    let mut grid = [[Type::Wall; N]; N];
    let mut things = Vec::new();
    for (r, line) in read_file(filename).enumerate() {
        for (c, v) in line.chars().enumerate() {
            match v {
                '#' => grid[r][c] = Type::Wall,
                '.' => grid[r][c] = Type::Empty,
                'G' => {
                    grid[r][c] = Type::Empty;
                    things.push(Thing {
                        pos: (r, c),
                        kind: Kind::Goblin,
                        attack: 3,
                        hp: 200,
                    });
                }
                'E' => {
                    grid[r][c] = Type::Empty;
                    things.push(Thing {
                        pos: (r, c),
                        kind: Kind::Elf,
                        attack: 3,
                        hp: 200,
                    });
                }
                _ => panic!("bad"),
            }
        }
    }
    let mut iter = 0;
    loop {
        things = things.into_iter().filter(|t| t.hp > 0).collect();
        things.sort_by_key(|t| t.pos);
        if debug {
            print(&grid, &things);
        }
        if tick(&grid, &mut things) {
            break;
        }
        iter += 1;
    }
    things = things.into_iter().filter(|t| t.hp > 0).collect();
    let sum: usize = things.iter().map(|t| t.hp).sum::<usize>();
    if debug {
        println!("iter: {} sum: {} iter*sum: {}  (iter-1)*sum: {}", iter, sum, iter*sum, (iter-1)*sum);
    }
    iter * sum
}

fn tick(grid: &[[Type; N]; N], things: &mut Vec<Thing>) -> bool {
    for i in 0..things.len() {
        if things[i].hp == 0 { continue; }
        if take_turn(i, grid, things) {
            return true;
        }
    }
    false
}

fn take_turn(idx: usize, grid: &[[Type; N]; N], things: &mut Vec<Thing>) -> bool {
    let this = things[idx];
    let targets = things.iter().cloned().filter(|o| o.kind != this.kind && o.hp > 0).collect::<Vec<_>>();
    if targets.is_empty() {
        return true;
    }
    let mut to_attack = Vec::new();
    for target in targets.iter() {
        if is_in_range(this.pos, target.pos) {
            to_attack.push(target);
        }
    }
    if to_attack.is_empty() {
        // move
        let mut locs = things.iter().cloned()
            .filter(|o| o.hp > 0)
            .map(|o| o.pos).collect::<HashSet<_>>();
        locs.remove(&this.pos);
        let mut nearests = Vec::new();
        let mut best = std::usize::MAX;
        for target in targets.iter() {
            let in_range = get_in_range(grid, target.pos, &locs);
            for point in in_range.iter() {
                if let Some(path) = find_path(this.pos, *point, grid, &locs) {
                    if path.len() == best {
                        nearests.push(path);
                    } else if path.len() < best {
                        nearests.clear();
                        best = path.len();
                        nearests.push(path);
                    }
                }
            }
        }
        if nearests.is_empty() {
            return false;
        }
        nearests.sort_by_key(|p| *p.back().unwrap());
        let destination_path = nearests.first().unwrap();
        let length = destination_path.len();
        let destination = destination_path.back().unwrap();
        for point in get_in_range(grid, this.pos, &locs) {
            if let Some(path) = find_path(point, *destination, grid, &locs) {
                if path.len() < length {
                    things[idx].pos = point;
                    break;
                }
            }
        }

        // maybe get targets
        for target in targets.iter() {
            if is_in_range(things[idx].pos, target.pos) {
                to_attack.push(target);
            }
        }
    }
    if to_attack.is_empty() {
        return false;
    }
    // attack
    to_attack.sort_by_key(|t| t.pos);
    let target = to_attack.iter().min_by_key(|t| t.hp).unwrap();
    for real_target in things.iter_mut().filter(|t| t.hp > 0) {
        if real_target.pos == target.pos {
            if real_target.hp < this.attack {
                real_target.hp = 0;
            } else {
                real_target.hp -= this.attack;
            }
            break;
        }
    }
    false
}

fn find_path(start: (usize, usize), goal: (usize, usize), grid: &[[Type; N]; N], locs: &HashSet<(usize, usize)>) -> Option<VecDeque<(usize, usize)>> {
    let mut stack = VecDeque::new();
    stack.push_back(start);
    let mut prev = HashMap::new();
    loop {
        if stack.is_empty() {
            return None;
        }
        let current = stack.pop_front().unwrap();
        if current == goal {
            let mut this = goal;
            let mut path = VecDeque::new();
            loop {
                if this == start {
                    return Some(path);
                }
                path.push_front(this);
                match prev.get(&this) {
                    Some(&v) => this = v,
                    None => return Some(path),
                }
            }
        }
        for ps in get_in_range(grid, current, locs).iter() {
            if !prev.contains_key(ps) {
                prev.insert(*ps, current);
                stack.push_back(*ps);
            }
        }
    }
}

#[inline]
fn is_in_range(a: (usize, usize), b: (usize, usize)) -> bool {
    (a.0 == b.0 && (a.1 == b.1 + 1 || a.1 + 1 == b.1))
        || (a.1 == b.1 && (a.0 == b.0 + 1 || a.0 + 1 == b.0))
}

fn get_in_range(grid: &[[Type; N]; N], pos: (usize, usize), locs: &HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut result = Vec::with_capacity(4);
    if pos.0 > 0 {
        let (r, c) = (pos.0 - 1, pos.1);
        if grid[r][c] != Type::Wall && !locs.contains(&(r, c)) {
            result.push((r,c));
        }
    }
    if pos.1 > 0 {
        let (r, c) = (pos.0, pos.1 - 1);
        if grid[r][c] != Type::Wall && !locs.contains(&(r, c)) {
            result.push((r,c));
        }
    }
    if pos.1 < N - 1 {
        let (r, c) = (pos.0, pos.1 + 1);
        if grid[r][c] != Type::Wall && !locs.contains(&(r, c)) {
            result.push((r,c));
        }
    }
    if pos.0 < N - 1 {
        let (r, c) = (pos.0 + 1, pos.1);
        if grid[r][c] != Type::Wall && !locs.contains(&(r, c)) {
            result.push((r,c));
        }
    }
    result
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Type {
    Wall,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Kind {
    Goblin,
    Elf,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Thing {
    pos: (usize, usize),
    kind: Kind,
    attack: usize,
    hp: usize,
}

use std::fmt;

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::Goblin => write!(f, "G"),
            Kind::Elf => write!(f, "E"),
        }
    }
}

impl fmt::Display for Thing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.kind, self.pos.1, self.pos.0, self.hp)
    }
}

#[allow(dead_code)]
fn print(grid: &[[Type; N]; N], things: &Vec<Thing>) {
    let ts = things.iter().map(|t| (t.pos, t)).collect::<HashMap<(usize, usize), _>>();
    for r in 0..N {
        for c in 0..N {
            if let Some(t) = ts.get(&(r, c)) {
                if t.kind == Kind::Goblin {
                    print!("G");
                } else {
                    print!("E");
                }
            } else {
                if grid[r][c] == Type::Wall {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }
        println!("");
    }
    println!("");
}
