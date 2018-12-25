extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Star {
    id: usize,
    pos: (i32, i32, i32, i32),
}

fn dist(a: &(i32, i32, i32, i32), b: &(i32, i32, i32, i32)) -> i32 {
    (a.0-b.0).abs()
        + (a.1-b.1).abs()
        + (a.2-b.2).abs()
        + (a.3-b.3).abs()
}

fn main() {
    let mut stars = Vec::new();
    for line in include_str!("../input.txt").lines() {
        let parts = line.split(',').map(|v| v.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let pos = (parts[0], parts[1], parts[2], parts[3]);
        let id = stars.len();
        stars.push(Star { id, pos });
    }

    let mut graph = HashMap::new();
    for star in stars.iter() {
        let others = stars
            .iter()
            .filter(|s| s.id != star.id && dist(&s.pos, &star.pos) <= 3)
            .map(|s| s.id)
            .collect::<Vec<usize>>();
        graph.insert(star.id, others);
    }

    let mut seen: HashSet<usize> = HashSet::new();
    let mut components = Vec::new();

    for i in 0..stars.len() {
        if seen.contains(&i) { continue; }

        let mut this = Vec::new();
        let mut queue = vec![i];
        while !queue.is_empty() {
            let cur = queue.pop().unwrap();
            this.push(cur);
            seen.insert(cur);

            for neigh in graph.get(&cur).unwrap().iter() {
                if seen.contains(neigh) {
                    continue;
                }
                queue.push(*neigh);
            }
        }
        components.push(this);
    }

    println!("{}", components.len());
}
