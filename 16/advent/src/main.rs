extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

fn main() {
    let mut lines = include_str!("../input.txt").lines();
    let mut counter = 0;
    let ops = vec![
        Op::Addr,
        Op::Addi,
        Op::Mulr,
        Op::Muli,
        Op::Banr,
        Op::Bani,
        Op::Borr,
        Op::Bori,
        Op::Setr,
        Op::Seti,
        Op::Gtir,
        Op::Gtri,
        Op::Gtrr,
        Op::Eqir,
        Op::Eqri,
        Op::Eqrr,
    ];
    loop {
        let before = lines.next().unwrap();
        let op = lines.next().unwrap();
        let after = lines.next().unwrap();
        if before.is_empty() && op.is_empty() {
            break;
        }
        lines.next();
        if check(before, op, after, &ops) {
            counter += 1;
        }
    }
    println!("{}", counter);
}

fn check(before: &str, o: &str, after: &str, ops: &Vec<Op>) -> bool {
    let before = before.trim_start_matches("Before: ").trim_start();
    let after = after.trim_start_matches("After: ").trim_start();

    let before = before.trim_start_matches("[").trim_end_matches("]");
    let start_reg = before.split(", ").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let after = after.trim_start_matches("[").trim_end_matches("]");
    let end_reg = after.split(", ").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let o = o.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let (op_code, a, b, c) = (o[0], o[1], o[2], o[3]);

    let mut op_count = 0;
    for op in ops.iter() {
        if op.execute(&start_reg, a, b, c) == end_reg {
            op_count += 1;
        }
    }
    op_count >= 3
}

#[derive(Debug)]
enum Op {
    Addr,
    Addi,
    Muli,
    Mulr,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Op {
    fn execute(&self, reg: &Vec<u32>, a: u32, b: u32, c: u32) -> Vec<u32> {
        let mut result = reg.clone();
        let val = match self {
            Op::Addi => result[a as usize] + b,
            Op::Addr => result[a as usize] + result[b as usize],
            Op::Mulr => result[a as usize] * result[b as usize],
            Op::Muli => result[a as usize] * b,
            Op::Banr => result[a as usize] & result[b as usize],
            Op::Bani => result[a as usize] & b,
            Op::Borr => result[a as usize] | result[b as usize],
            Op::Bori => result[a as usize] | b,
            Op::Setr => result[a as usize],
            Op::Seti => a,
            Op::Gtir => if a > result[b as usize] { 1 } else { 0 },
            Op::Gtri => if result[a as usize] > b { 1 } else { 0 },
            Op::Gtrr => if result[a as usize] > result[b as usize] { 1 } else { 0 },
            Op::Eqir => if a == result[b as usize] { 1 } else { 0 },
            Op::Eqri => if result[a as usize] == b { 1 } else { 0 },
            Op::Eqrr => if result[a as usize] == result[b as usize] { 1 } else { 0 },
        };
        result[c as usize] = val;
        result
    }
}
