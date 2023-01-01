use std::fs::File;
use std::io::Write;
use tempdir::TempDir;
use toy_core::prelude::*;
use toy_plugin_file::config::ReadConfig;
use toy_plugin_file::service::Read;

#[tokio::test]
async fn read_glob() {
    let root = preapre_temp();

    let mut f = File::create("test-1.csv").unwrap();
    writeln!(f, "column1,column2").unwrap();
    writeln!(f, "a,b").unwrap();
    let mut f = File::create("test-2.csv").unwrap();
    writeln!(f, "column1,column2").unwrap();
    writeln!(f, "c,d").unwrap();

    let data = vec![
        map_value!("column1" => &b"a"[..], "column2" => &b"b"[..]),
        map_value!("column1" => &b"c"[..], "column2" => &b"d"[..]),
    ];

    let result = read(&root, "test*csv").await;

    assert_eq!(result.get(0).unwrap(), data.get(0).unwrap());
    assert_eq!(result.get(1).unwrap(), data.get(1).unwrap());
}

async fn read(root: &TempDir, file_name: &str) -> Vec<Value> {
    let mut service = Read;
    let (tx, mut rx) = toy_core::mpsc::channel(100);
    let task_ctx = toy_plugin_test::dummy_task_context();
    let config = ReadConfig::new(format!(
        "{}{}{}",
        root.path().display(),
        std::path::MAIN_SEPARATOR,
        file_name
    ));

    let mut c = service
        .new_context(toy_plugin_test::dummy_service_type(), config)
        .await
        .unwrap();

    loop {
        let frame = Frame::default();
        let r = service
            .handle(task_ctx.clone(), c, frame.clone(), tx.clone())
            .await
            .unwrap();
        match r {
            ServiceContext::Complete(r) => {
                c = r.into();
                break;
            }
            _ => c = r.into(),
        };
    }

    let r = service
        .upstream_finish_all(task_ctx.clone(), c, tx.clone())
        .await;
    assert!(r.is_ok());
    drop(tx);

    let mut result = vec![];
    while let Some(item) = rx.next().await {
        result.push(item.value().unwrap().clone());
    }
    result
}

fn preapre_temp() -> TempDir {
    let root = TempDir::new("toy-reader-test");
    let root = root.ok().expect("Should have created a temp directory");
    assert!(std::env::set_current_dir(root.path()).is_ok());
    root
}
