use crate::{tree, Node};
use bit_vec::BitVec;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{self, prelude::*, BufReader},
};

// the compressed binary file uses the following format
// 1 u8 for knowing number of padding bits at the end
// 1 u16 for the number of bits used by the serialized tree
// the rest of the bits (minus padding) is the encoded text
// for example:
//   padding: 5    tree: 29                 tree                  text    padding
//   (00000101)(0000000000011101)(01011000010100001010101100010)(011010)(00000)

pub fn compress(input_file: &str, output_file_name: &str) -> io::Result<()> {
    let mut file = File::create(output_file_name.to_owned() + ".dat")?;
    let nodes = get_external_nodes(input_file).unwrap();
    let tree = tree::from(nodes);
    let encodings = get_encodings(&tree);

    let mut encoded = tree::serialize(&tree);

    let tree_len: u16 = encoded
        .len()
        .try_into()
        .expect("Tree size exceeded limit of u16");

    let mut text = encode_file(input_file, &encodings).unwrap();

    encoded.append(&mut text);

    let padding: u8 = (8 - (encoded.len() % 8))
        .try_into()
        .expect("Padding exceeded limit of u8");

    file.write_all(&[padding])?;
    file.write_all(&tree_len.to_be_bytes())?;
    file.write_all(&encoded.to_bytes())?;

    Ok(())
}

fn encode_file(file_name: &str, encodings: &HashMap<u8, BitVec>) -> io::Result<BitVec> {
    let f = File::open(file_name)?;
    let f = BufReader::new(f);
    let mut bv = BitVec::new();

    for line in f.lines() {
        for b in (line.unwrap() + "\n").bytes() {
            if let Some(encoding) = encodings.get(&b) {
                bv.extend(encoding.iter());
            }
        }
    }

    Ok(bv)
}

fn get_external_nodes(file_name: &str) -> io::Result<BinaryHeap<Reverse<Node>>> {
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
