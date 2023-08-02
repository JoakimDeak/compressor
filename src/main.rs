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
mod serializer;

// next steps
// encode the file with a binary representation
// write that to a file

// look up serialization of a tree
// come up with a binary format for that
// write a decoder
// write a tree constructor from binary rep
// move tree constructor to own module

// for the encodings map
// create a depth first search and with each step push to a BitVec
// once we reach and external node we add that bitvec to a hashmap where the key is the byte and the value is the bitvec
// once we've done that for every external we should have a full encoding map

fn main() {
    let start = Instant::now();
    let nodes = get_external_nodes_heap("src/input.txt").unwrap();
    let elapsed = start.elapsed();
    let tree = construct_tree(nodes);
    let encodings = get_encodings(&tree);
    println!("{:#?}", encodings);
    println!("{:?}", elapsed);
}

fn get_external_nodes_heap(file_name: &str) -> io::Result<BinaryHeap<Reverse<Node>>> {
    let f = File::open(file_name)?;
    let f = BufReader::new(f);
    let mut nodes = HashMap::new();

    for line in f.lines() {
        for b in (line.unwrap() + "\n").bytes() {
            nodes.entry(b).or_insert(Node::new_external(b, 0)).count += 1;
        }
    }
    Ok(nodes.into_values().map(Reverse).collect())
}

fn construct_tree(mut nodes: BinaryHeap<Reverse<Node>>) -> Node {
    while nodes.len() > 1 {
        let left = nodes.pop().unwrap().0;
        let right = nodes.pop().unwrap().0;
        let internal = Node::new_internal(left, right);
        nodes.push(Reverse(internal));
    }
    nodes.pop().unwrap().0
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
