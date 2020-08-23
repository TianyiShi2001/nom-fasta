# nom-fasta

FASTA parser implemented in Rust with nom.

[**nom**](https://github.com/Geal/nom) is a performant yet easy-to-use parser combinators library written in Rust. **nom** can handle either text or binary data. Here, based on **nom**, I implemented a parser for the FASTA, a text file format containing one more more protein/DNA/RNA sequences.

# Example

```rust
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
```

```
[Ok(Record { id: "gi|2765644|emb|Z78519.1|CPZ78519", desc: Some("C.pubescens 5.8S rRNA gene and ITS1 and ITS2 DNA"), seq: "ATATGATCGAGTGAATCTGGTGGACTTGTGGTTACTCAGCTCGCCATAGGCTTTGCTTTTGCGGTGACCCTAATTTGTCATTGGGCCTCCTCCCAAGCTTTCCTTGTGGGTTTGAACCTCTAGCACGGTGCAGTA" }), Ok(Record { id: "gi|2765643|emb|Z78518.1|CRZ78518", desc: Some("C.reginae 5.8S rRNA gene and ITS1 and ITS2 DNA"), seq: "CGTAACAAGGTTTCCGTAGGTGAACCTGCGGGAGGATCATTGTTGAGATAGTAGAATATTCGATCGAGTGAATCCGGAGGACTTGTGGTTACTCGGCTCGTCGAAGGCTTAACTTTTGTGGTGACCCTGATTTGT" }), Ok(Record { id: "gi|2765642|emb|Z78517.1|CFZ78517", desc: Some("C.flavum 5.8S rRNA gene and ITS1 and ITS2 DNA"), seq: "CGTAACAAGGTTTCCGTAGGTGAACCTGCGGAAGGATCATTGTTGA...

// --snip-- //

...TTAGTTGGGCC" })]
```

# Comparison With `bio::io::fasta`

As of 2020-08-24, `bio::io::fasta` is faster.

```
test tests::bench_read_bio_io_fasta ... bench:     157,047 ns/iter (+/- 19,803)
test tests::bench_read_my_fasta     ... bench:     467,353 ns/iter (+/- 53,507)
```