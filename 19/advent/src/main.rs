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

    let mut ip: u128 = 0;
    let mut regs: Vec<u128> = vec![0, 0, 0, 0, 0, 0];
    while (ip as usize) < instrs.len() {
        regs[ip_reg] = ip;
        let instr = instrs[ip as usize];
        let new_regs = instr.0.execute(&regs, instr.1, instr.2, instr.3);
        regs = new_regs;
        ip = regs[ip_reg];
        ip += 1;
    }
    println!("Part 1: {}", regs[0]);

    // After printing out the registers and seeing the loop,
    // I played around with jumping the registers ahead based
    // on the conditions I could see that would move things.
    // Eventually I saw that the conditions:
    //    mulr 1 4 5
    //    eqrr 5 2 5
    // was really driving everything as register 2 was set to
    // 10551305 and was not moving. Register 5 would increment
    // by 1 and then when that condition was true, we would add
    // register 1 to register 0 and start over with one larger
    // in register 1. If that multiplication was impossible to
    // get equality then we would eventually loop back around
    // when 5 got bigger than 2.
    //
    // So we are just adding up the divisors of the number in
    // register 2. So without doing anything fancy this loop
    // does the same thing:
    let mut sum = 0;
    for i in 1..(10551305+1) {
        if 10551305 % i == 0 {
            sum += i;
        }
    }
    println!("Part 2: {}", sum);
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
