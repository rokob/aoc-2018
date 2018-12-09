extern crate slab;
extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

use slab::Slab;
use std::ops::{Index, IndexMut};

// 405 players; last marble is worth 71700 points
const PLAYERS: usize = 405;
const LAST: usize = 7170000;

fn main() {
    let mut marbles: List<usize> = List::with_capacity(LAST);
    let mut current = marbles.push_front(0);
    let mut scores = HashMap::new();
    let mut player = 1;

    for to_play in 1..=LAST {
        if to_play % 23 == 0 {
            let e = scores.entry(player).or_insert(0);
            *e += to_play;
            for _ in 0..7 {
                current = marbles[current].prev;
            }
            let tmp = marbles[current].next;
            let val = marbles.remove(current);
            *e += val;
            current = tmp;
        } else {
            current = marbles[current].next;
            current = marbles.insert_after(current, to_play);
        }
        player = if player == PLAYERS { 1 } else { player + 1 };
    }

    let result = scores.iter().map(|(_, v)| v).max().unwrap();
    println!("{}", result);
}

struct List<T> {
    slab: Slab<Node<T>>,
    head: Pointer,
}

struct Node<T> {
    value: T,
    next: Pointer,
    prev: Pointer,
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Pointer(usize);

impl Pointer {
    #[inline]
    fn null() -> Pointer {
        Pointer(!0)
    }
    #[inline]
    fn is_null(&self) -> bool {
        *self == Pointer::null()
    }
}

impl<T> Index<Pointer> for List<T> {
    type Output = Node<T>;

    fn index(&self, index: Pointer) -> &Node<T> {
        &self.slab[index.0]
    }
}

impl<T> IndexMut<Pointer> for List<T> {
    fn index_mut(&mut self, index: Pointer) -> &mut Node<T> {
        &mut self.slab[index.0]
    }
}

impl<T> List<T> {
    fn new() -> List<T> {
        List {
            slab: Slab::new(),
            head: Pointer::null(),
        }
    }

    fn with_capacity(cap: usize) -> List<T> {
        List {
            slab: Slab::with_capacity(cap),
            head: Pointer::null(),
        }
    }

    fn push_front(&mut self, t: T) -> Pointer {
        let head = self.head;
        if head.is_null() {
            let n = Pointer(self.slab.insert(Node {
                value: t,
                prev: Pointer::null(),
                next: Pointer::null(),
            }));
            self.head = n;
            let node = &mut self[n];
            node.prev = n;
            node.next = n;
            n
        } else {
            self.insert_before(head, t)
        }
    }

    fn insert_after(&mut self, node: Pointer, t: T) -> Pointer {
        let next = self[node].next;
        let n = Pointer(self.slab.insert(Node {
            value: t,
            prev: node,
            next: next,
        }));
        self[next].prev = n;
        self[node].next = n;
        n
    }

    fn insert_before(&mut self, node: Pointer, t: T) -> Pointer {
        let prev = self[node].prev;
        let n = Pointer(self.slab.insert(Node {
            value: t,
            prev: prev,
            next: node,
        }));
        self[prev].next = n;
        self[node].prev = n;
        n
    }

    fn remove(&mut self, node: Pointer) -> T {
        let prev = self[node].prev;
        let next = self[node].next;
        self[prev].next = next;
        self[next].prev = prev;
        self.slab.remove(node.0).value
    }
}
