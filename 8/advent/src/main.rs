extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};

struct Node {
    children: Vec<Box<Node>>,
    metadata: Vec<usize>,
}


fn main() {
    for line in read_file("input.txt") {
        let parts = split_ws(&line);
        let parts = parts.iter().map(|c| c.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let root = get_nodes(&parts, 0);
        println!("{}", count_meta(&root.0));
    }
}

fn count_meta(node: &Node) -> usize {
    let mut sum = 0;
    for n in node.children.iter() {
        sum += count_meta(n);
    }
    for m in node.metadata.iter() {
        sum += *m;
    }
    sum
}

fn get_nodes(data: &[usize], start: usize) -> (Node, usize) {
    let (children, metadata) = (data[start], data[start+1]);
    if children == 0 {
        return (
            Node {
                children: vec![],
                metadata: data[start+2..start+2+metadata].iter().cloned().collect(),
            },
            start+2+metadata
            );
    }
    let mut childs = Vec::new();
    let mut offset = start+2;
    for c in 0..children {
        let (child, next) = get_nodes(data, offset);
        childs.push(Box::new(child));
        offset = next;
    }
    (
        Node {
            children: childs,
            metadata: data[offset..offset+metadata].iter().cloned().collect(),
        },
        offset+metadata
    )
}
