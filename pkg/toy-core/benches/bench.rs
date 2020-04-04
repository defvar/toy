#![feature(test)]

extern crate test;

use test::black_box;
use test::test::Bencher;
use toy_core::prelude::Value;

#[bench]
fn value_parse_str(b: &mut Bencher) {
    let v = Value::from(1u32);
    b.iter(|| {
        for _ in 0..100 {
            let s = v.parse_str();
            black_box(s);
        }
    });
}

#[bench]
fn value_parse_str_std(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..100 {
            let s = Value::from(1u32.to_string());
            black_box(s);
        }
    });
}
