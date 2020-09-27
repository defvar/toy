#![feature(test)]

extern crate test;

use std::future::Future;
use std::task::Context;
use test::black_box;
use test::test::Bencher;
use tokio::macros::support::{Pin, Poll};
use toy_core::prelude::Value;

#[bench]
fn call_dynamic(b: &mut Bencher) {
    let mut v: Box<dyn Add> = Box::new(Counter(0));
    b.iter(|| {
        for _ in 0..10000 {
            let r = v.add();
            black_box(r);
        }
    });
}

#[bench]
fn call_static(b: &mut Bencher) {
    let mut v = Counter(0);
    b.iter(|| {
        for _ in 0..10000 {
            let r = v.add();
            black_box(r);
        }
    });
}

trait Add {
    fn add(&mut self) -> u32;
    fn clear(&mut self);
}

struct Counter(u32);

impl Add for Counter {
    fn add(&mut self) -> u32 {
        self.0 += 1;
        self.0
    }

    fn clear(&mut self) {
        self.0 = 0;
    }
}
