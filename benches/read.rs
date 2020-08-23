#![feature(test)]

extern crate bio;
extern crate fasta;
extern crate test;

use bio::io::fasta as bf;
use fasta::record;
use fasta::BoxError;
use fasta::Records;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn read_my_fasta() {
    let mut f = File::open("e.fasta").unwrap();
    let mut data = String::with_capacity(512);
    f.read_to_string(&mut data).unwrap();
    let a: Vec<_> = Records(&data).into_iter().collect();
}

fn read_bio_io_fasta() {
    let mut reader = bf::Reader::from_file("e.fasta").unwrap();
    let a: Vec<_> = reader.records().into_iter().collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_read_my_fasta(b: &mut Bencher) {
        b.iter(|| read_my_fasta());
    }
    #[bench]
    fn bench_read_bio_io_fasta(b: &mut Bencher) {
        b.iter(|| read_bio_io_fasta());
    }
}
