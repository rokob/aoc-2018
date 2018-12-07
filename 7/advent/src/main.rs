extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

const WORKERS: usize = 5;

fn main() {
    let mut prereqs = HashMap::new();
    let mut befores = HashSet::new();
    let mut afters = HashSet::new();
    for line in read_file("input.txt") {
        let parts = split_ws(&line);
        let before = parts[1].chars().next().unwrap();
        let after = parts[7].chars().next().unwrap();
        befores.insert(before.clone());
        afters.insert(after.clone());
        let mut e = prereqs.entry(after).or_insert(Vec::new());
        e.push(before);
        e.sort();
    }

    let mut can_do = Vec::new();
    for b in befores.iter() {
        if !afters.contains(&b) {
            can_do.push(*b);
        }
    }
    can_do.sort_by_key(|&c| std::cmp::Reverse(c));

    let mut workers = [(' ', 0); WORKERS];

    let mut counter = 0;
    loop {
        let mut finished = Vec::new();
        let mut avail;
        loop {
            avail = 0;
            for w in 0..WORKERS {
                if workers[w].1 <= 1 {
                    if workers[w].0 != ' ' {
                        finished.push(workers[w].0);
                        workers[w].0 = ' ';
                        workers[w].1 = 0;
                    }
                    avail += 1;
                }
            }
            if avail > 0 {
                break;
            }
            for w in 0..WORKERS {
                workers[w].1 -= 1;
            }
            counter += 1;
        }

        for (c, ps) in prereqs.iter_mut() {
            for current in finished.iter() {
                let idx_r = ps.binary_search(&current);
                if let Ok(idx) = idx_r {
                    ps.remove(idx);
                    if ps.is_empty() {
                        can_do.push(*c);
                    }
                }
            }
        }
        can_do.sort_by_key(|&c| std::cmp::Reverse(c));

        let mut to_start = Vec::with_capacity(avail);
        for _ in 0..avail {
            if can_do.is_empty() {
                break;
            }
            to_start.push(can_do.pop().unwrap());
        }

        let mut work_idx = 0;
        for i in 0..WORKERS {
            if workers[i].1 == 0 {
                if to_start.len() > work_idx {
                    let work = to_start[work_idx];
                    work_idx += 1;
                    workers[i].0 = work;
                    workers[i].1 = (work as u8 - 'A' as u8) + 1 + 60;
                }
            } else {
                workers[i].1 -= 1;
            }
        }
        if can_do.is_empty() && !work_todo(&workers) {
            break;
        }
        counter += 1;
    }
    println!("{}", counter);
}

fn work_todo(work: &[(char, u8); WORKERS]) -> bool {
    for (_, t) in work {
        if *t > 0 {
            return true;
        }
    }
    false
}
