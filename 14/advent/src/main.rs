extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

const N: usize = 919901;

fn main() {
    let mut v = vec![3, 7, 1, 0];
    let mut a = 0;
    let mut b = 1;

    while v.len() < N + 11 {
        let sum = v[a] + v[b];
        if sum > 9 {
            v.push(1);
            v.push(sum % 10);
        } else {
            v.push(sum);
        }
        a = (a + 1 + v[a]) % v.len();
        b = (b + 1 + v[b]) % v.len();
    }

    println!("{:?}", &v[N..N+10]);
}
