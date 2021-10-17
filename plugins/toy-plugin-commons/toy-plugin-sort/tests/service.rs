use toy_core::prelude::*;
use toy_plugin_sort::config::{BufferFullStrategy, SortConfig, SortKey};
use toy_plugin_sort::service::Sort;

#[tokio::test]
async fn test_sort_by_value() {
    let data = vec![
        Value::from(7),
        Value::from(6),
        Value::from(5),
        Value::from(3),
        Value::from(4),
        Value::from(9),
        Value::from(1),
        Value::from(2),
        Value::from(8),
    ];

    let r = sort(&data, SortKey::Value).await;
    for i in 0..9 {
        assert_eq!(r.get(i).unwrap().value().unwrap(), (i + 1) as u32);
    }
}

#[tokio::test]
async fn test_sort_by_name() {
    let data = vec![
        map_value!("a" => 1, "b" => "9"), //max
        map_value!("a" => 3, "b" => "7"), //min
        map_value!("a" => 2, "b" => "8"), //mid
    ];

    let r = sort(&data, SortKey::Name("b".to_string())).await;
    assert_eq!(r.get(0).unwrap().value().unwrap(), data.get(1).unwrap());
    assert_eq!(r.get(1).unwrap().value().unwrap(), data.get(2).unwrap());
    assert_eq!(r.get(2).unwrap().value().unwrap(), data.get(0).unwrap());
}

async fn sort(data: &Vec<Value>, key: SortKey) -> Vec<Frame> {
    let mut service = Sort;
    let (tx, mut rx) = toy_core::mpsc::channel(10);
    let task_ctx = toy_plugin_test::dummy_task_context();

    // let config = SortConfig::new(
    //     5,
    //     BufferFullStrategy::Persist {
    //         temp_path: "/tmp/toy-plugin-sort-test".into(),
    //     },
    // );
    let config = SortConfig::with(10, BufferFullStrategy::Flush, key);
    let mut c = service
        .new_context(toy_plugin_test::dummy_service_type(), config)
        .await
        .unwrap();

    for v in data {
        let frame = Frame::from_value(v.clone());
        let r = service
            .handle(task_ctx.clone(), c, frame.clone(), tx.clone())
            .await;
        assert!(r.is_ok());
        c = r.unwrap().into();
    }

    let r = service
        .upstream_finish_all(task_ctx.clone(), c, tx.clone())
        .await;
    assert!(r.is_ok());
    drop(tx);

    let mut result = vec![];
    while let Some(Ok(item)) = rx.next().await {
        println!("{:?}", item);
        result.push(item);
    }

    result
}
