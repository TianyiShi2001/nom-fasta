# nom-fasta

FASTA parser implemented in Rust with nom.

[**nom**](https://github.com/Geal/nom) is a performant yet easy-to-use parser combinators library written in Rust. **nom** can handle either text or binary data. Here, based on **nom**, I implemented a parser for the FASTA, a text file format containing one more more protein/DNA/RNA sequences.

# Comparison With `bio::io::fasta`

As of 2020-08-24, `bio::io::fasta` is faster.

```
test tests::bench_read_bio_io_fasta ... bench:     157,047 ns/iter (+/- 19,803)
test tests::bench_read_my_fasta     ... bench:     467,353 ns/iter (+/- 53,507)
```