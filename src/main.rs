use bit_vec::BitVec;
use node::Node;
use std::{
    fs::read,
    io::{self},
    time::Instant,
};

use crate::tree::serialize;

mod compressor;
mod node;
mod tree;

// next steps
// decompress the compressed file

fn main() -> io::Result<()> {
    let start = Instant::now();
    compressor::compress("src/small_input.txt", "output")?;
    // read_file()?;

    let elapsed = start.elapsed();
    println!("{:?}", elapsed);
    Ok(())
}

fn read_file() -> io::Result<()> {
    let res = BitVec::from_bytes(&read("output.dat")?);
    println!("out {:#?}", res);
    Ok(())
}
