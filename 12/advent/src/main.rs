extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

#[derive(Debug)]
struct Rule(bool, bool, bool, bool, bool, bool);

impl Rule {
    pub fn new(left: &str, result: &str) -> Self {
        let chars = left.chars().collect::<Vec<char>>();
        let result = result.chars().next().unwrap() == '#';
        Rule(
            chars[0] == '#',
            chars[1] == '#',
            chars[2] == '#',
            chars[3] == '#',
            chars[4] == '#',
            result)
    }

    fn match_(&self, parts: &[bool]) -> (bool, bool) {
        (self.0 == parts[0]
         && self.1 == parts[1]
         && self.2 == parts[2]
         && self.3 == parts[3]
         && self.4 == parts[4],
         self.5)
    }
}

const N: usize = 20;
const SIZE: usize = N*100;

fn main() {
    let mut lines = read_file("input.txt");
    let initial = lines.next().unwrap();
    let initial = split_ws(&initial)[2];
    lines.next();

    println!("inital: {}", initial.len());
    let mut state = [false; SIZE];
    let zero = SIZE / 2;
    for (i, c) in initial.chars().enumerate() {
        state[zero + i] = c == '#';
    }

    let mut rules = Vec::new();

    for rule in lines {
        let parts = split_ws(&rule);
        let r = Rule::new(parts[0], parts[2]);
        rules.push(r);
    }

    let mut iter = 0;
    loop {
        let (next, same) = do_round(&state, &rules);
        if same || iter > 200 {
            break;
        }
        print(&state[zero-1..zero+105]);
        state = next;
        iter += 1;
    }
    let result = count(&state, zero);
    println!("{}", result);
}

fn print(state: &[bool]) {
    for &c in state.iter() {
        if c { print!("#") } else { print!(".") }
    }
    println!("");
}

fn count(state: &[bool], zero: usize) -> isize {
    let mut result = 0;
    for (i, &b) in state.iter().enumerate() {
        if b {
            result += i as isize - zero as isize;
        }
    }
    result
}

fn do_round(state: &[bool], rules: &Vec<Rule>) -> ([bool; SIZE], bool) {
    let mut same = true;
    let mut next = [false; SIZE];
    let mut i = 2;
    while i + 2 < state.len() {
        for rule in rules {
            let (m, r) = rule.match_(&state[i-2..i+3]);
            if m {
                next[i] = r;
                if state[i] != next[i] {
                    same = false;
                }
                break;
            }
        }
        i += 1;
    }
    (next, same)
}
