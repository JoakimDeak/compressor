use crate::{BitVec, Node};
use bit_vec::Iter;
use std::{cmp::Reverse, collections::BinaryHeap};

pub fn from(mut nodes: BinaryHeap<Reverse<Node>>) -> Node {
    while nodes.len() > 1 {
        let left = nodes.pop().unwrap().0;
        let right = nodes.pop().unwrap().0;
        let internal = Node::new_internal(left, right);
        nodes.push(Reverse(internal));
    }
    nodes.pop().unwrap().0
}

pub fn serialize(tree: &Node) -> BitVec {
    let mut bv = BitVec::new();
    serialize_rec(&tree, &mut bv);
    bv
}

fn serialize_rec(node: &Node, bv: &mut BitVec) {
    if node.byte.is_some() {
        bv.push(true);
        serialize_byte(node.byte.unwrap(), bv);
    } else {
        bv.push(false);
        if node.left.is_some() {
            serialize_rec(node.left.as_ref().unwrap(), bv);
        }
        if node.right.is_some() {
            serialize_rec(node.right.as_ref().unwrap(), bv);
        }
    }
}

pub fn deserialize(bv: BitVec) -> Node {
    let mut iter = bv.iter();
    deserialize_rec(&mut iter).unwrap()
}

fn deserialize_rec(iter: &mut Iter<'_>) -> Option<Node> {
    match iter.next() {
        Some(true) => Some(Node::new_external_deserialized(deserialize_byte(iter))),
        Some(false) => Some(Node::new_internal_deserialized(
            deserialize_rec(iter).unwrap(),
            deserialize_rec(iter).unwrap(),
        )),
        None => None,
    }
}

fn serialize_byte(byte: u8, bv: &mut BitVec) {
    for i in 0..8 {
        bv.push(((1 << (7 - i)) & byte) > 0);
    }
}

fn deserialize_byte(iter: &mut Iter<'_>) -> u8 {
    let res = BitVec::from_fn(8, |_| iter.next().unwrap());
    res.to_bytes()[0]
}
