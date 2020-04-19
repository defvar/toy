#![feature(test)]

extern crate test;

use std::io::{self, Read};
use test::black_box;
use test::test::Bencher;

use toy_pack_mp::{decoder_from_reader, DecoderOps};

static TEST_DATA: &'static str = "benches/reader.txt";

#[bench]
fn reader_no_buf(b: &mut Bencher) {
    b.iter(|| {
        let f = std::fs::File::open(TEST_DATA).unwrap();
        let mut decoder = decoder_from_reader(f);
        for _ in 0..100 {
            let r = decoder.get_bytes(10).unwrap();
            black_box(r);
        }
    })
}

#[bench]
fn reader_buf(b: &mut Bencher) {
    b.iter(|| {
        let f = std::fs::File::open(TEST_DATA).unwrap();
        let buf = io::BufReader::with_capacity(1024, f);
        let mut decoder = decoder_from_reader(buf);
        for _ in 0..100 {
            let r = decoder.get_bytes(10).unwrap();
            black_box(r);
        }
    })
}

#[bench]
fn reader_std(b: &mut Bencher) {
    b.iter(|| {
        let f = std::fs::File::open(TEST_DATA).unwrap();
        let mut decoder = io::BufReader::with_capacity(1024, f);
        let mut buf = [0u8; 10];
        for _ in 0..100 {
            let r = decoder.read_exact(&mut buf).unwrap();
            black_box(r);
        }
    })
}
