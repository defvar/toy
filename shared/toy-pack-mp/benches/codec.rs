#![feature(test)]

extern crate test;

use std::io;
use test::test::Bencher;

use rmp::decode::read_int;
use rmp::encode::write_uint;
use rmp::Marker;

use toy_pack_mp::marker::marker_from_byte;
use toy_pack_mp::{decoder_from_slice, encoder_from_writer, DecoderOps, EncoderOps};

#[bench]
fn write_uint_toy(b: &mut Bencher) {
    let mut vec: Vec<u8> = Vec::new();
    let mut encoder = encoder_from_writer(&mut vec);
    let _ = marker_from_byte(0u8);

    b.iter(|| {
        for i in 0..100 {
            encoder.encode_uint(i).unwrap();
        }
    })
}

#[bench]
fn write_uint_rmp(b: &mut Bencher) {
    let mut vec: Vec<u8> = Vec::new();
    b.iter(|| {
        for i in 0..100 {
            write_uint(&mut vec, i).unwrap();
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

#[bench]
fn marker_loopkup_array(b: &mut Bencher) {
    // prepare "lazy static"
    let _ = marker_from_byte(0u8);
    b.iter(|| {
        for i in 0..255 {
            let _ = marker_from_byte(i);
        }
    })
}

#[bench]
fn marker_loopkup_match(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..255 {
            let _ = Marker::from_u8(i);
        }
    })
}
