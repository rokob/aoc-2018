extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

fn main() {
    for line in read_file("input.txt") {
        let parts = split_ws(&line);
        let parts = parts.iter().map(|c| c.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let root = get_nodes(&parts, 0);
        println!("{}", count_meta(&root.0));
        println!("{}", score_node(&root.0));
    }
}

fn score_node(node: &Node) -> usize {
    if node.children.is_empty() {
        return node.metadata.iter().sum();
    }
    let mut score = 0;
    for &m in node.metadata.iter() {
        if m == 0 || m > node.children.len() {
            continue;
        }
        score += score_node(&node.children[m - 1]);
    }
    score
}

fn count_meta(node: &Node) -> usize {
    let mut sum = 0;
    for n in node.children.iter() {
        sum += count_meta(n);
    }
    sum + node.metadata.iter().sum::<usize>()
}

fn get_nodes(data: &[usize], start: usize) -> (Node, usize) {
    let (child_count, metadata) = (data[start], data[start+1]);
    if child_count == 0 {
        return (
            Node {
                children: vec![],
                metadata: data[start+2..start+2+metadata].iter().cloned().collect(),
            },
            start+2+metadata
            );
    }
    let mut children = Vec::new();
    let mut offset = start+2;
    for _ in 0..child_count {
        let (child, next) = get_nodes(data, offset);
        children.push(child);
        offset = next;
    }
    (
        Node {
            children,
            metadata: data[offset..offset+metadata].iter().cloned().collect(),
        },
        offset+metadata
    )
}
