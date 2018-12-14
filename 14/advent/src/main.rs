extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

const SEQ: &'static [usize] = &[9, 1, 9, 9, 0, 1];
const N: usize = 6;

fn main() {
    let mut v = vec![3, 7, 1, 0, 1, 0, 1, 2];
    let mut a = 0;
    let mut b = 6;

    let offset = loop {
        let l = v.len();
        if &v[l - N..l] == SEQ {
            break 0;
        }
        if &v[l - N - 1..l - 1] == SEQ {
            break 1;
        }

        let sum = v[a] + v[b];
        if sum > 9 {
            v.push(1);
            v.push(sum % 10);
        } else {
            v.push(sum);
        }
        a = (a + 1 + v[a]) % v.len();
        b = (b + 1 + v[b]) % v.len();
    };

    println!("{}", v.len() - N - offset);
}
