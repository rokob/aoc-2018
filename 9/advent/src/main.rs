extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

// 405 players; last marble is worth 71700 points
const PLAYERS: usize = 405;
const LAST: usize = 71700;

fn main() {
    let mut marbles = vec![0];
    let mut current = 0;
    let mut to_place = 1;
    let mut scores = HashMap::new();
    let mut player = 0;

    while !take_turn(&mut marbles, &mut to_place, &mut current, &mut scores, &mut player) {}

    println!("{:?}", scores.iter().map(|(_, v)| v).max());
}

fn take_turn(marbles: &mut Vec<usize>, to_place: &mut usize, current: &mut usize, scores: &mut HashMap<usize, usize>, player: &mut usize) -> bool {
    if *to_place > LAST {
        return true;
    }
    if *to_place % 23 == 0 {
        let e = scores.entry(*player).or_insert(0);
        *e += *to_place;
        let remove_idx = if *current < 7 {
            marbles.len() - (7 - *current)
        } else {
            *current - 7
        };
        *e += marbles.remove(remove_idx);
        *player = (*player + 1) % PLAYERS;
        *to_place += 1;
        *current = remove_idx;
        return false;
    } else {
        let place_idx = (*current + 2) % marbles.len();
        marbles.insert(place_idx, *to_place);
        *player = (*player + 1) % PLAYERS;
        *to_place += 1;
        *current = place_idx;
        return false;
    }
}
