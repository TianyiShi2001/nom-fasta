extern crate bio;
extern crate fasta;
use bio::io::fasta as bf;
use fasta::record;
use fasta::BoxError;
use fasta::Records;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), BoxError> {
    // let fp = std::env::args().nth(1).unwrap();
    // let mut f = File::open(fp)?;
    // let mut data = String::with_capacity(512);
    // f.read_to_string(&mut data)?;
    // for r in Records(&data) {
    //     println!("{:?}", r);
    // }
    read_me();
    //read_bio();
    println!(
        "{}",
        "---------------------------\n\n\n\n-------------------------"
    );
    read_bio();
    Ok(())
}

fn read_me() {
    let mut f = File::open("e.fasta").unwrap();
    let mut data = String::with_capacity(512);
    f.read_to_string(&mut data).unwrap();
    let a: Vec<_> = Records(&data).into_iter().collect();
    println!("{:?}", a)
}

fn read_bio() {
    let mut reader = bf::Reader::from_file("e.fasta").unwrap();
    let a: Vec<_> = reader.records().into_iter().collect();
    println!("{:?}", a);
}
