extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

const N: usize = 150;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
enum Move {
    Left,
    Straight,
    Right,
}

impl Move {
    fn next(&self) -> Self {
        match self {
            Move::Left => Move::Straight,
            Move::Straight => Move::Right,
            Move::Right => Move::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
struct Cart {
    id: usize,
    loc: (usize, usize),
    dir: Dir,
    next_move: Move,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
enum Path {
    Empty,
    Horiz,
    Vert,
    Intersection,
    Curve(char),
}

fn main() {
    let mut grid = [[Path::Empty; N]; N];
    let mut carts = Vec::new();
    for (r, line) in read_file("input.txt").enumerate() {
        for (c, part) in line.chars().enumerate() {
            match part {
                '/' => grid[r][c] = Path::Curve('/'),
                '\\' => grid[r][c] = Path::Curve('\\'),
                '-' => grid[r][c] = Path::Horiz,
                '|' => grid[r][c] = Path::Vert,
                '+' => grid[r][c] = Path::Intersection,
                '^' => {
                    grid[r][c] = Path::Vert;
                    let cart = Cart {
                        id: carts.len(),
                        loc: (r, c),
                        dir: Dir::Up,
                        next_move: Move::Left,
                    };
                    carts.push(cart);
                },
                'v' => {
                    grid[r][c] = Path::Vert;
                    let cart = Cart {
                        id: carts.len(),
                        loc: (r, c),
                        dir: Dir::Down,
                        next_move: Move::Left,
                    };
                    carts.push(cart);
                },
                '>' => {
                    grid[r][c] = Path::Horiz;
                    let cart = Cart {
                        id: carts.len(),
                        loc: (r, c),
                        dir: Dir::Right,
                        next_move: Move::Left,
                    };
                    carts.push(cart);
                },
                '<' => {
                    grid[r][c] = Path::Horiz;
                    let cart = Cart {
                        id: carts.len(),
                        loc: (r, c),
                        dir: Dir::Left,
                        next_move: Move::Left,
                    };
                    carts.push(cart);
                },
                ' ' => {},
                _ => panic!("bad input: {}", part),
            }
        }
    }

    let mut iter = 0;
    let result = loop {
        carts.sort_by_key(|c| c.loc);
        iter += 1;
        let prev_locations = carts.iter().map(|c| c.loc).collect::<HashSet<_>>();
        if let Some(loc) = tick(&grid, &mut carts, &prev_locations) {
            break loc;
        }
        let mut crashes = check_for_crash(&carts);
        if !crashes.is_empty() {
            crashes.sort();
            break crashes[0];
        }
    };
    println!("iter: {}", iter);
    println!("{},{}", result.1, result.0);
}

fn check_for_crash(carts: &Vec<Cart>) -> Vec<(usize, usize)> {
    let mut locs = HashSet::new();
    let mut crashes = Vec::new();
    for c in carts.iter() {
        if !locs.insert(&c.loc) {
            crashes.push(c.loc);
        }
    }
    crashes
}

fn tick(grid: &[[Path; N]; N], carts: &mut Vec<Cart>, prev: &HashSet<(usize, usize)>) -> Option<(usize, usize)> {
    for c in carts.iter_mut() {
        let next_loc = match c.dir {
            Dir::Up => (c.loc.0 - 1, c.loc.1),
            Dir::Down => (c.loc.0 + 1, c.loc.1),
            Dir::Left => (c.loc.0, c.loc.1 - 1),
            Dir::Right => (c.loc.0, c.loc.1 + 1),
        };
        match grid[next_loc.0][next_loc.1] {
            Path::Empty => panic!("bad grid"),
            Path::Horiz | Path::Vert => {},
            Path::Intersection => {
                let next_dir = match c.next_move {
                    Move::Straight => c.dir,
                    Move::Left => match c.dir {
                        Dir::Up => Dir::Left,
                        Dir::Down => Dir::Right,
                        Dir::Left => Dir::Down,
                        Dir::Right => Dir::Up,
                    },
                    Move::Right => match c.dir {
                        Dir::Up => Dir::Right,
                        Dir::Down => Dir::Left,
                        Dir::Left => Dir::Up,
                        Dir::Right => Dir::Down,
                    }
                };
                let next_move = c.next_move.next();
                c.dir = next_dir;
                c.next_move = next_move;
            },
            Path::Curve(v) => {
                let next_dir = match c.dir {
                    Dir::Up => if v == '/' { Dir::Right } else { Dir::Left },
                    Dir::Down => if v == '/' { Dir::Left } else { Dir::Right },
                    Dir::Left => if v == '/' { Dir::Down } else { Dir::Up },
                    Dir::Right => if v == '/' { Dir::Up } else { Dir::Down },
                };
                c.dir = next_dir;
            },
        }
        c.loc = next_loc;
        if prev.contains(&c.loc) {
            return Some(c.loc);
        }
    }
    None
}

#[allow(dead_code)]
fn print_around(grid: &[[Path; N]; N], cart: &Cart) {
    for r in std::cmp::max(0, cart.loc.0 - 3)..std::cmp::min(N, cart.loc.0 + 3) {
        for c in std::cmp::max(0, cart.loc.1 - 3)..std::cmp::min(N, cart.loc.1 + 3) {
            if (r, c) == cart.loc {
                match cart.dir {
                    Dir::Up => print!("^"),
                    Dir::Down => print!("v"),
                    Dir::Right => print!(">"),
                    Dir::Left => print!("<"),
                }
            } else {
                match grid[r][c] {
                    Path::Empty => print!(" "),
                    Path::Curve(c) => print!("{}", c),
                    Path::Horiz => print!("-"),
                    Path::Vert => print!("|"),
                    Path::Intersection => print!("+"),
                }
            }
        }
        println!("");
    }
    println!("");
}
