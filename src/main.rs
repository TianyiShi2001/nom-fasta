extern crate fasta;
use fasta::Records;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("e.fasta").unwrap();
    let mut data = String::with_capacity(512);
    f.read_to_string(&mut data).unwrap();
    let a: Vec<_> = Records(&data).into_iter().collect();
    println!("{:?}", a);
}
