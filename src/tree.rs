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

// serialization does a depth first traversal
// if the current node is internal then we write the bit 0 and make recursive calls for left and right
// if the current node is external we write the bit 1 followed by the byte of the node
// given this tree
//      O
//     / \
//   97   O
//       / \
//     10   98
// serialization would be: 01(01100001)01(00001010)1(01100010)
// parentheses showing the bytes 97, 10, and 98

pub fn serialize(tree: &Node) -> BitVec {
    let mut bv = BitVec::new();
    serialize_rec(&tree, &mut bv);
    bv
}

fn serialize_rec(node: &Node, bv: &mut BitVec) {
    match node.byte {
        Some(byte) => {
            bv.push(true);
            serialize_byte(byte, bv);
        }
        None => {
            bv.push(false);
            if let Some(left) = &node.left {
                serialize_rec(left.as_ref(), bv);
            }
            if let Some(right) = &node.right {
                serialize_rec(right.as_ref(), bv);
            }
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
