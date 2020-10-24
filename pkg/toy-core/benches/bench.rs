#![feature(test)]

extern crate test;

use test::black_box;
use test::test::Bencher;

#[bench]
fn test_join_key_string(b: &mut Bencher) {
    let parent = Some("aiueo".to_string());
    b.iter(|| {
        for i in (0..100).map(|x| x.to_string()) {
            let text = Some(
                parent
                    .clone()
                    .map_or(i.to_string(), |x| format!("{}.{}", x, i)),
            );
            black_box(text);
        }
    });
}

#[bench]
fn test_join_key_byte(b: &mut Bencher) {
    let parent = {
        let mut vec = Vec::<u8>::new();
        vec.extend_from_slice("aiueo".as_bytes());
        vec
    };
    b.iter(|| {
        for i in (0..100).map(|x| x.to_string()) {
            let mut text = Vec::<u8>::new();
            if parent.len() > 0 {
                text.extend_from_slice(parent.as_slice());
                text.push(b'.');
            }
            text.extend_from_slice(i.as_bytes());
            black_box(&text);
            text.clear();
        }
    });
}

#[bench]
fn test_call_dynamic(b: &mut Bencher) {
    let mut v: Box<dyn Add> = Box::new(Counter(0));
    b.iter(|| {
        for _ in 0..10000 {
            let r = v.add();
            black_box(r);
        }
    });
}

#[bench]
fn test_call_static(b: &mut Bencher) {
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
