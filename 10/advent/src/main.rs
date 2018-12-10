extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

struct Point {
    pos: (isize, isize),
    vel: (isize, isize),
}

fn main() {
    let mut points = Vec::new();
    for line in read_file("input.txt") {
        let point = get_point(&line);
        points.push(point);
    }

    let mut best_area = move_points(&mut points, 5000);
    for n in 5000..100000 {
        let area = move_points(&mut points, 1);
        if area < best_area {
            best_area = area;
            display_points(&points);
        } else {
            println!("n => {}", n);
            break;
        }
    }
}

fn get_point(line: &str) -> Point {
    let pos = &line[10..24];
    let pos: Vec<isize> = pos
        .split(", ")
        .map(|v| v.trim().parse::<isize>().unwrap())
        .collect();
    let pos = (pos[0], pos[1]);
    let vel = &line[36..42];
    let vel: Vec<isize> = vel
        .split(", ")
        .map(|v| v.trim().parse::<isize>().unwrap())
        .collect();
    let vel = (vel[0], vel[1]);

    Point { pos, vel }
}

fn display_points(points: &Vec<Point>) {
    let (min_x, min_y, max_x, max_y, area) = get_size(points);
    if area > 3000 {
        return;
    }

    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            if points.iter().any(|p| p.pos == (x, y)) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    println!("");
}

fn get_size(points: &Vec<Point>) -> (isize, isize, isize, isize, isize) {
    let min_x = points.iter().map(|p| p.pos.0).min().unwrap();
    let min_y = points.iter().map(|p| p.pos.1).min().unwrap();
    let max_x = points.iter().map(|p| p.pos.0).max().unwrap();
    let max_y = points.iter().map(|p| p.pos.1).max().unwrap();

    (
        min_x,
        min_y,
        max_x,
        max_y,
        (max_y - min_y) * (max_x - min_x),
    )
}

fn move_points(points: &mut Vec<Point>, amt: isize) -> isize {
    for p in points.iter_mut() {
        p.pos = (p.pos.0 + amt * p.vel.0, p.pos.1 + amt * p.vel.1);
    }

    get_size(points).4
}
