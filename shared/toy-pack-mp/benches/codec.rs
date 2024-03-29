#![feature(test)]

extern crate test;

use std::io;
use test::test::Bencher;

use rmp::decode::read_int;
use rmp::encode::{write_sint, write_str, write_uint};

use toy_pack_mp::marker::marker_from_byte;
use toy_pack_mp::{decoder_from_slice, encoder_from_writer, DecoderOps, EncoderOps};

#[bench]
fn write_uint_toy(b: &mut Bencher) {
    let mut vec: Vec<u8> = Vec::new();
    let mut encoder = encoder_from_writer(&mut vec);
    let _ = marker_from_byte(0u8);

    b.iter(|| {
        for i in 0..100 {
            encoder.encode_uint(i as u64).unwrap();
        }
    })
}

#[bench]
fn write_uint_rmp(b: &mut Bencher) {
    let mut vec: Vec<u8> = Vec::new();
    b.iter(|| {
        for i in 0..100 {
            write_uint(&mut vec, i as u64).unwrap();
        }
    })
}

#[bench]
fn write_sint_toy(b: &mut Bencher) {
    let mut vec: Vec<u8> = Vec::new();
    let mut encoder = encoder_from_writer(&mut vec);
    let _ = marker_from_byte(0u8);

    b.iter(|| {
        for i in 0..100 {
            encoder.encode_sint(i as i64).unwrap();
        }
    })
}

#[bench]
fn write_sint_rmp(b: &mut Bencher) {
    let mut vec: Vec<u8> = Vec::new();
    b.iter(|| {
        for i in 0..100 {
            write_sint(&mut vec, i as i64).unwrap();
        }
    })
}

#[bench]
fn write_str_toy(b: &mut Bencher) {
    let mut vec: Vec<u8> = Vec::new();
    let mut encoder = encoder_from_writer(&mut vec);
    let _ = marker_from_byte(0u8);

    b.iter(|| {
        for _ in 0..100 {
            encoder.encode_str("aiueo").unwrap();
        }
    })
}

#[bench]
fn write_str_rmp(b: &mut Bencher) {
    let mut vec: Vec<u8> = Vec::new();
    b.iter(|| {
        for _ in 0..100 {
            write_str(&mut vec, "aiueo").unwrap();
        }
    })
}

fn read_int_bench_src() -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    for _ in 0..1000 {
        vec.push(0xcd as u8);
        vec.push(0);
        vec.push(0);
    }
    vec
}

#[bench]
fn read_int_toy_slice(b: &mut Bencher) {
    let _ = marker_from_byte(0u8);

    b.iter(|| {
        let src = read_int_bench_src();
        let mut decoder = decoder_from_slice(&src[..]);
        for _ in 0..1000 {
            assert_eq!(0u16, decoder.decode_integer().unwrap());
        }
    })
}

#[bench]
fn read_int_rmp(b: &mut Bencher) {
    b.iter(|| {
        let src = read_int_bench_src();
        let mut cur = io::Cursor::new(&src[..]);
        for _ in 0..1000 {
            assert_eq!(0u16, read_int(&mut cur).unwrap());
        }
    })
}
