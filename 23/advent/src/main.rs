extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Bot {
    pos: (isize, isize, isize),
    radius: isize,
}

fn main() {
    let mut bots = Vec::new();
    let mut key = (0, 0, 0); 
    let mut best = 0;
    for line in include_str!("../input.txt").lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let pos_part = parts[0];
        let radius = parts[1].split("=").collect::<Vec<_>>()[1].parse::<isize>().unwrap();
        let pos_nums = &pos_part[5..pos_part.len()-2];
        let pos_nums = pos_nums.split(',').map(|n| n.parse::<isize>().unwrap()).collect::<Vec<_>>();
        let pos = (pos_nums[0], pos_nums[1], pos_nums[2]);
        if radius > best {
            best = radius;
            key = pos;
        }
        bots.push(Bot{pos, radius});
    }

    let mut count = 0;
    for bot in bots.iter() {
        if is_in_range(&key, &bot.pos, best) {
            count += 1;
        }
    }
    println!("Part 1: {}", count);

    let result = solve(&bots);
    println!("Part 2: {}", result.1);
}

fn is_in_range(a: &(isize, isize, isize), b: &(isize, isize, isize), radius: isize) -> bool {
    let distance = (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs();
    distance <= radius
}

fn dist(a: &(isize, isize, isize), b: &(isize, isize, isize)) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()
}


fn solve(bots: &Vec<Bot>) -> ((isize, isize, isize), isize) {
    let mut xs = (bots.iter().map(|b| b.pos.0).min().unwrap(), bots.iter().map(|b| b.pos.0).max().unwrap());
    let mut ys = (bots.iter().map(|b| b.pos.1).min().unwrap(), bots.iter().map(|b| b.pos.1).max().unwrap());
    let mut zs = (bots.iter().map(|b| b.pos.2).min().unwrap(), bots.iter().map(|b| b.pos.2).max().unwrap());

    let mut d = 1;
    while d < xs.1 - xs.0 {
        d *= 2;
    }

    loop {
        let mut best_n = 0;
        let mut best = (0, 0, 0);
        let mut best_val = 0;
        for x in (xs.0..=xs.1).step_by(d as usize) {
            for y in (ys.0..=ys.1).step_by(d as usize) {
                for z in (zs.0..=zs.1).step_by(d as usize) {
                    let mut count = 0;
                    for bot in bots.iter() {
                        let this_dist = dist(&(x, y, z), &bot.pos);
                        if (this_dist - bot.radius) / d <= 0 {
                            count += 1;
                        }
                    }
                    if count > best_n {
                        best_n = count;
                        best = (x, y, z);
                        best_val = x.abs() + y.abs() + z.abs();
                    } else if count == best_n {
                        let val = x.abs() + y.abs() + z.abs();
                        if val < best_val {
                            best = (x, y, z);
                            best_val = val;
                        }
                    }
                }
            }
        }
        if d == 1 {
            return (best, best_val);
        }
        xs = (best.0 - d, best.0 + d);
        ys = (best.1 - d, best.1 + d);
        zs = (best.2 - d, best.2 + d);
        d /= 2;
    }
}
