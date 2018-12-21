extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut seen = HashSet::new();
    let mut last = 0u128;
    let mut x_1: u128;
    let mut x_3 = 0u128;
    let mut x_4: u128;
    let x_0 = 0;
    'a: loop {
      x_4 = x_3 | 65536;
      x_3 = 2176960;
      'b: loop {
        x_1 = x_4 & 255;
        x_3 = x_3 + x_1;
        x_3 = x_3 & 16777215;
        x_3 = x_3 * 65899;
        x_3 = x_3 & 16777215;
        if 256 > x_4 {
          if x_3 == x_0 {
              println!("oops");
            break 'a;
          } else {
              if !seen.insert(x_3) {
                  break 'a;
              } else {
                  last = x_3;
              }
            continue 'a;
          }
        }
        x_4 = x_4 / 256;
      }
    }
    println!("Part 2: {}", last);
}

fn part1() {
    let mut lines = include_str!("../input.txt").lines();
    let ip_reg = lines.next().unwrap().split(" ").collect::<Vec<_>>()[1].parse::<usize>().unwrap();

    let mut instrs = Vec::new();
    for line in lines {
        instrs.push(parse(line));
    }

let mut x_3: u128 = 0;
let mut x_4 = x_3 | 65536;
x_3 = 2176960;
let mut x_1 = x_4 & 255;
x_3 = x_3 + x_1;
x_3 = x_3 & 16777215;
x_3 = x_3 * 65899;
x_3 = x_3 & 16777215;

x_4 = 256;

x_1 = x_4 & 255;
x_3 = x_3 + x_1;
x_3 = x_3 & 16777215;
x_3 = x_3 * 65899;
x_3 = x_3 & 16777215;

x_4 = 1;

x_1 = x_4 & 255;
x_3 = x_3 + x_1;
x_3 = x_3 & 16777215;
x_3 = x_3 * 65899;
x_3 = x_3 & 16777215;

    let mut ip: u128 = 0;
    let mut regs: Vec<u128> = vec![x_3, 0, 0, 0, 0, 0];
    while (ip as usize) < instrs.len() {
        regs[ip_reg] = ip;
        let instr = instrs[ip as usize];
        let new_regs = instr.0.execute(&regs, instr.1, instr.2, instr.3);
        regs = new_regs;
        ip = regs[ip_reg];
        ip += 1;
    }

    println!("Part 1: {}", x_3);
}

fn parse(line: &str) -> (Op, u128, u128, u128) {
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
    let a = vals[1].parse::<u128>().unwrap();
    let b = vals[2].parse::<u128>().unwrap();
    let c = vals[3].parse::<u128>().unwrap();
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
}
use Op::*;

impl Op {
    fn execute(&self, reg: &Vec<u128>, a: u128, b: u128, c: u128) -> Vec<u128> {
        let mut result = reg.clone();
        result[c as usize] = match self {
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
