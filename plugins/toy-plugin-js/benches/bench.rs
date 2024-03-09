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
    request.payload.message2 = request.payload.message;
    delete request.payload.message;
    request;
    "#;
    let config = FunctionConfig {
        name: "test".to_string(),
        code: code.to_string(),
    };

    let (tx, mut rx) = toy_core::mpsc::channel(10);
    let task_ctx = toy_plugin_test::dummy_task_context();

    let rt = tokio::runtime::Runtime::new().unwrap();

    b.iter(|| {
        rt.block_on(async {
            let c = new_function_context(toy_plugin_test::dummy_service_type(), config.clone())
                .unwrap();
            let mut c = ServiceContext::Ready(c);
            for _ in 0..1000 {
                let tx2 = tx.clone();
                let frame2 = frame.clone();
                let task_ctx2 = task_ctx.clone();
                c = js_function(task_ctx2, c.into(), frame2, tx2).await.unwrap();
                let r = rx.next().await;
                let _ = black_box(r);
            }
        });
    });
}
