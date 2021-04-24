#![feature(test, min_type_alias_impl_trait)]

extern crate test;

use async_trait::async_trait;
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
