#![feature(test)]

extern crate test;

use std::io::BufRead;
use test::black_box;
use test::test::Bencher;
use toy_core::prelude::*;
use toy_plugin_sort::config::{BufferFullStrategy, SortConfig, SortKey};
use toy_plugin_sort::service::Sort;

const TMP_PATH: &str = "/tmp/toy-plugin-sort-bench";
const TEST_FILE: &str = "./benches/data.txt";

#[bench]
fn bench_sort(b: &mut Bencher) {
    let _ = std::fs::create_dir(TMP_PATH);

    let config = SortConfig::with(
        10000,
        BufferFullStrategy::Persist {
            path: TMP_PATH.into(),
        },
        SortKey::Value,
    );
    let (tx, mut rx) = toy_core::mpsc::channel(10000);
    let task_ctx = toy_plugin_test::dummy_task_context();
    let rt = tokio::runtime::Runtime::new().unwrap();

    // receive thread
    rt.spawn(async move {
        let mut count = 0u64;
        let mut all_count = 0u64;
        while let Some(Ok(item)) = rx.next().await {
            count += 1;
            if count >= 10000 {
                all_count += count;
                //println!("{}", all_count);
                count = 0;
            }
        }
    });
    b.iter(|| {
        let tx2 = tx.clone();
        rt.block_on(async {
            let c = Sort
                .new_context(toy_plugin_test::dummy_service_type(), config.clone())
                .await
                .unwrap();
            let mut c = ServiceContext::Ready(c);
            let file = std::fs::File::open(TEST_FILE).unwrap();
            let mut reader = std::io::BufReader::new(file);
            let mut buf = String::new();
            let mut count = 0u64;
            let mut all_count = 0u64;
            while let Ok(size) = reader.read_line(&mut buf) {
                if size == 0 {
                    break;
                }
                let digit = buf.replace("\n", "").parse::<i64>();
                if digit.is_err() {
                    println!("{:?},{:?}", buf, digit.err());
                    panic!()
                }
                let frame = Frame::from(digit.unwrap());
                let task_ctx2 = task_ctx.clone();
                c = Sort
                    .handle(task_ctx2, c.into(), frame, tx2.clone())
                    .await
                    .unwrap();
                count += 1;
                if count >= 10000 {
                    all_count += count;
                    //println!("handle:{}", all_count);
                    count = 0;
                }

                buf.clear();
            }
            let r = Sort
                .upstream_finish_all(task_ctx.clone(), c.into(), tx2.clone())
                .await;
            let _ = black_box(r);
        });
        drop(tx2)
    });
    drop(tx)
}
