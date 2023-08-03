use bit_vec::BitVec;
use node::Node;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{self, prelude::*, BufReader},
    time::Instant,
};

mod node;
mod tree;

// next steps
// encode the file with a binary representation
// write that to a file

fn main() {
    let start = Instant::now();
    let nodes = get_external_nodes_heap("src/small_input.txt").unwrap();
    let tree = tree::from(nodes);
    println!("before {:#?}", tree);
    let res = tree::serialize(&tree);
    let tree = tree::deserialize(res);
    println!("after {:#?}", tree);
    let elapsed = start.elapsed();
    println!("{:?}", elapsed);
}

fn get_external_nodes_heap(file_name: &str) -> io::Result<BinaryHeap<Reverse<Node>>> {
    let f = File::open(file_name)?;
    let f = BufReader::new(f);
    let mut nodes = HashMap::new();

    for line in f.lines() {
        for b in (line.unwrap() + "\n").bytes() {
            nodes
                .entry(b)
                .or_insert(Node::new_external(b, 0))
                .increment_count()
        }
    }
    Ok(nodes.into_values().map(Reverse).collect())
}

fn get_encodings(node: &Node) -> HashMap<u8, BitVec> {
    let mut encodings: HashMap<u8, BitVec> = HashMap::new();
    let encoding = BitVec::new();
    get_encodings_rec(&node, &mut encodings, encoding);
    encodings
}

fn get_encodings_rec(node: &Node, encodings: &mut HashMap<u8, BitVec>, encoding: BitVec) {
    if node.byte.is_some() {
        encodings.insert(node.byte.unwrap(), encoding);
        return;
    }

    if node.left.is_some() {
        let mut encoding = encoding.clone();
        encoding.push(false);
        get_encodings_rec(node.left.as_ref().unwrap(), encodings, encoding)
    }

    if node.right.is_some() {
        let mut encoding = encoding.clone();
        encoding.push(true);
        get_encodings_rec(node.right.as_ref().unwrap(), encodings, encoding);
    }
}
