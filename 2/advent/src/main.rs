extern crate utils;

fn main() {
    let mut lines: Vec<String> = Vec::new();
    for line in utils::get_lines("advent.txt") {
        let line = line.unwrap();
        lines.push(line);
    }

    for (i, line) in lines.iter().enumerate() {
        for (j, o) in lines.iter().enumerate() {
            if i == j { continue; }
            let mut diff: u32 = 0;
            for (a, b) in line.chars().zip(o.chars()) {
                if a != b {
                    diff += 1;
                }
                if diff > 1 {
                    break;
                }
            }
            if diff == 1 {
                for (a, b) in line.chars().zip(o.chars()) {
                    if a == b {
                        print!("{}", a);
                    }
                }
                println!("");
                return;
            }
        }
    }

}

