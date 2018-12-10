extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

use std::collections::VecDeque;

// 405 players; last marble is worth 71700 points
const PLAYERS: usize = 405;
const LAST: usize = 7170000;

fn rotate<T>(v: &mut VecDeque<T>, amt: isize) {
    if amt == 0 || v.len() < 2 {
        return;
    }
    if amt < 0 {
        for _ in amt..0 {
            let elem = v.pop_front().unwrap();
            v.push_back(elem);
        }
    } else {
        for _ in 0..amt {
            let elem = v.pop_back().unwrap();
            v.push_front(elem);
        }
    }
}

fn main() {
    let result = compute(PLAYERS, LAST);
    println!("{}", result);
}

fn compute(players: usize, max: usize) -> usize {
    let mut marbles = VecDeque::with_capacity(max);
    marbles.push_front(0);
    let mut scores = HashMap::new();
    let mut player = 1;

    for to_play in 1..=max {
        if to_play % 23 == 0 {
            let e = scores.entry(player).or_insert(0);
            *e += to_play;
            rotate(&mut marbles, 7);
            let val = marbles.pop_back().unwrap();
            rotate(&mut marbles, -1);
            *e += val;
        } else {
            rotate(&mut marbles, -1);
            marbles.push_back(to_play);
        }
        player = if player == players { 1 } else { player + 1 };
    }
    scores.iter().map(|(_, v)| *v).max().unwrap()
}

#[test]
fn test_examples() {
    /*
    10 players; last marble is worth 1618 points: high score is 8317
    13 players; last marble is worth 7999 points: high score is 146373
    17 players; last marble is worth 1104 points: high score is 2764
    21 players; last marble is worth 6111 points: high score is 54718
    30 players; last marble is worth 5807 points: high score is 37305
    */

    assert_eq!(compute(10, 1618), 8317);
    assert_eq!(compute(13, 7999), 146373);
    assert_eq!(compute(17, 1104), 2764);
    assert_eq!(compute(21, 6111), 54718);
    assert_eq!(compute(30, 5807), 37305);
}
