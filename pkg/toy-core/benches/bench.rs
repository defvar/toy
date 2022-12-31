#![feature(test, type_alias_impl_trait)]

extern crate test;

use async_trait::async_trait;
use std::sync::Arc;
use test::black_box;
use test::test::Bencher;

#[bench]
fn test_clone_no_arc(b: &mut Bencher) {
    let v = DummyNonArc {
        v: get_clone_data(),
    };
    b.iter(|| {
        let mut vec = Vec::<DummyNonArc>::new();
        for _ in 0..100 {
            vec.push(v.clone());
        }
        black_box(vec);
    });
}

#[bench]
fn test_clone_arc(b: &mut Bencher) {
    let v = DummyArc {
        v: Arc::new(get_clone_data()),
    };
    b.iter(|| {
        let mut vec = Vec::<DummyArc>::new();
        for _ in 0..100 {
            vec.push(v.clone());
        }
        black_box(vec);
    });
}

#[bench]
fn test_format_string_single_write(b: &mut Bencher) {
    b.iter(|| {
        let mut vec = Vec::<u8>::new();
        for i in (0..100).map(|x| x.to_string()) {
            let r = std::io::Write::write(&mut vec, format!("\"{}\"", i).as_bytes()).unwrap();
            black_box(r);
        }
    });
}

#[bench]
fn test_format_byte_multi_write(b: &mut Bencher) {
    b.iter(|| {
        let mut vec = Vec::<u8>::new();
        for i in (0..100).map(|x| x.to_string()) {
            let mut r = std::io::Write::write(&mut vec, &[b'\"']).unwrap();
            r += std::io::Write::write(&mut vec, i.as_bytes()).unwrap();
            r += std::io::Write::write(&mut vec, &[b'\"']).unwrap();
            black_box(r);
        }
    });
}

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
        let mut text = Vec::<u8>::new();
        for i in (0..100).map(|x| x.to_string()) {
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
fn test_async_call_dynamic(b: &mut Bencher) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    b.iter(|| {
        let mut v = get_add_box();
        rt.block_on(async {
            let mut ctx = Context { count: 0 };
            for _ in 0..10000 {
                ctx = v.add(ctx).await;
                ctx = black_box(ctx);
            }
        });
    });
}

#[bench]
fn test_async_call_static(b: &mut Bencher) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    b.iter(|| {
        let mut v = get_add_static();
        rt.block_on(async move {
            let mut ctx = Context { count: 0 };
            for _ in 0..10000 {
                ctx = v.add(ctx).await;
                ctx = black_box(ctx);
            }
        });
    });
}

fn get_add_box() -> impl AsyncAddBox {
    CounterBox
}

fn get_add_static() -> impl AsyncAddStatic {
    CounterStatic
}

struct Context {
    count: u32,
}

#[async_trait]
trait AsyncAddBox {
    async fn add(&mut self, ctx: Context) -> Context;
}

trait AsyncAddStatic {
    type Future: std::future::Future<Output = Context> + Send;

    fn add(&mut self, ctx: Context) -> Self::Future;
}

struct CounterBox;
struct CounterStatic;

#[async_trait]
impl AsyncAddBox for CounterBox {
    async fn add(&mut self, mut ctx: Context) -> Context {
        ctx.count += 1;
        ctx
    }
}

impl AsyncAddStatic for CounterStatic {
    type Future = impl std::future::Future<Output = Context> + Send;

    fn add(&mut self, mut ctx: Context) -> Self::Future {
        async {
            ctx.count += 1;
            ctx
        }
    }
}

#[derive(Debug, Clone)]
struct Record {
    a: u64,
    b: String,
    c: String,
    d: f64,
}

#[derive(Debug, Clone)]
struct DummyArc {
    v: Arc<Vec<Record>>,
}

#[derive(Debug, Clone)]
struct DummyNonArc {
    v: Vec<Record>,
}

fn get_clone_data() -> Vec<Record> {
    let mut r = vec![];
    for _ in 0..100 {
        let item = Record {
            a: 999,
            b: "b".repeat(100),
            c: "c".repeat(100),
            d: 999.9,
        };
        r.push(item);
    }
    r
}
