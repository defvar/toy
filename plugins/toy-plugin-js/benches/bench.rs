#![feature(test)]

extern crate test;

use test::black_box;
use test::test::Bencher;
use toy_core::prelude::*;
use toy_plugin_js::config::FunctionConfig;
use toy_plugin_js::service::{js_function, new_function_context};

#[bench]
fn bench_function(b: &mut Bencher) {
    let value = map_value! {
        "message" => "a",
        "number" => 0,
    };
    let frame = Frame::from_value(value);

    let code = r#"
    let req = toy();
    req.message2 = req.message;
    delete req.message;
    req;
    "#;
    let config = FunctionConfig {
        code: code.to_string(),
    };

    let (tx, mut rx) = toy_core::mpsc::channel(10);
    let task_ctx = toy_plugin_test::dummy_task_context();
    let c = new_function_context(toy_plugin_test::dummy_service_type(), config).unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();

    b.iter(|| {
        rt.block_on(async {
            for _ in 0..100 {
                let tx2 = tx.clone();
                let c2 = c.clone();
                let frame2 = frame.clone();
                let task_ctx2 = task_ctx.clone();
                let r = js_function(task_ctx2, c2, frame2, tx2).await;
                let _ = rx.next().await;
                let _ = black_box(r);
            }
        });
    });
}
