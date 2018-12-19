extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

fn main() {
    let mut lines = include_str!("../input.txt").lines();
    let ip_reg = lines.next().unwrap().split(" ").collect::<Vec<_>>()[1].parse::<usize>().unwrap();

    let mut instrs = Vec::new();
    for line in lines {
        instrs.push(parse(line));
    }

    let mut ip: u32 = 0;
    let mut regs: Vec<u32> = vec![0, 0, 0, 0, 0, 0];
    while (ip as usize) < instrs.len() {
        regs[ip_reg] = ip;
        let instr = instrs[ip as usize];
        let new_regs = instr.0.execute(&regs, instr.1, instr.2, instr.3);
        regs = new_regs;
        ip = regs[ip_reg];
        ip += 1;
    }
    println!("{}", regs[0]);
}

fn parse(line: &str) -> (Op, u32, u32, u32) {
    let vals = line.split(" ").collect::<Vec<_>>();
    let op = match vals[0] {
                "addr" => Addr,
                "addi" => Addi,
                "muli" => Muli,
                "mulr" => Mulr,
                "banr" => Banr,
                "bani" => Bani,
                "borr" => Borr,
                "bori" => Bori,
                "setr" => Setr,
                "seti" => Seti,
                "gtir" => Gtir,
                "gtri" => Gtri,
                "gtrr" => Gtrr,
                "eqir" => Eqir,
                "eqri" => Eqri,
                "eqrr" => Eqrr,
                _ => panic!("bad"),
    };
    let a = vals[1].parse::<u32>().unwrap();
    let b = vals[2].parse::<u32>().unwrap();
    let c = vals[3].parse::<u32>().unwrap();
    (op, a, b, c)
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
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
    Unknown,
}
use Op::*;

impl Op {
    fn execute(&self, reg: &Vec<u32>, a: u32, b: u32, c: u32) -> Vec<u32> {
        let mut result = reg.clone();
        result[c as usize] = match self {
            Op::Unknown => panic!("bad op"),
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
            Op::Gtir => if a > result[b as usize] {
                1
            } else {
                0
            },
            Op::Gtri => if result[a as usize] > b {
                1
            } else {
                0
            },
            Op::Gtrr => if result[a as usize] > result[b as usize] {
                1
            } else {
                0
            },
            Op::Eqir => if a == result[b as usize] {
                1
            } else {
                0
            },
            Op::Eqri => if result[a as usize] == b {
                1
            } else {
                0
            },
            Op::Eqrr => if result[a as usize] == result[b as usize] {
                1
            } else {
                0
            },
        };
        result
    }
}
