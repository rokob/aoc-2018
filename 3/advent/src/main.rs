extern crate utils;
#[allow(unused_imports)]
use utils::{HashSet, HashMap, read_file, split_ws};

fn main() {
    let mut matrix = [[0; 1200]; 1200];
    let mut count = 0;
    for line in read_file("advent.txt") {
        let claim = split_ws(&line);
        let pos = claim[2];
        let size = claim[3];
        let pos = pos[0..pos.len()-1].split(',').map(|v| v.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let (x, y) = (pos[0], pos[1]);

        let size = size.split('x').map(|v| v.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let (w, h) = (size[0], size[1]);

        for r in x..x+w {
            for c in y..y+h {
                matrix[r][c] += 1;
                if matrix[r][c] == 2 {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);

    for line in read_file("advent.txt") {
        let claim = split_ws(&line);
        let pos = claim[2];
        let size = claim[3];
        let pos = pos[0..pos.len()-1].split(',').map(|v| v.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let (x, y) = (pos[0], pos[1]);

        let size = size.split('x').map(|v| v.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let (w, h) = (size[0], size[1]);

        let mut cool = true;
        for r in x..x+w {
            for c in y..y+h {
                if matrix[r][c] > 1 {
                    cool = false;
                }
            }
        }
        if cool {
            println!("{}", claim[0]);
            break;
        }
    }
}
