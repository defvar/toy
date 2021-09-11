#![feature(test)]

extern crate test;

use test::black_box;
use test::test::Bencher;
use toy_core::prelude::*;
use toy_plugin_map::config::MappingConfig;
use toy_plugin_map::service::{mapping, new_mapping_context};

#[bench]
fn bench_mapping(b: &mut Bencher) {
    let value = map_value! {
        "message" => "a",
        "number" => 0,
    };
    let frame = Frame::from_value(value);

    let mappings = {
        let mut map = Map::new();
        map.insert("message".to_string(), "message2".to_string());
        map
    };
    let config = MappingConfig { mappings };

    let (tx, mut rx) = toy_core::mpsc::channel(10);
    let task_ctx = toy_plugin_test::dummy_task_context();
    let c = new_mapping_context(toy_plugin_test::dummy_service_type(), config).unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();

    b.iter(|| {
        rt.block_on(async {
            for _ in 0..100 {
                let tx2 = tx.clone();
                let c2 = c.clone();
                let frame2 = frame.clone();
                let task_ctx2 = task_ctx.clone();
                let r = mapping(task_ctx2, c2, frame2, tx2).await;
                let _ = rx.next().await;
                let _ = black_box(r);
            }
        });
    });
}
