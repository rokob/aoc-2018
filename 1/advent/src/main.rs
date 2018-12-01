extern crate utils;

fn main() {
    for (i, line) in utils::get_lines("advent.txt").enumerate() {
        let line = line.unwrap();
        println!("{}: {}", i, line);
    }
}
