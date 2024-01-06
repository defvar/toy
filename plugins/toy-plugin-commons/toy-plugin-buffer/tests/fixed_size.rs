use toy_core::prelude::*;
use toy_plugin_buffer::config::FixedSizeConfig;
use toy_plugin_buffer::service::FixedSize;

#[tokio::test]
async fn chunked() {
    let data = seq_value![1, 2, 3, 4, 5];
    let r = go(data, 3).await;

    {
        let map = r.get(0).unwrap().as_map().unwrap();
        assert_eq!(
            map.get("payload").unwrap(),
            &Value::from(vec![Value::from(1), Value::from(2), Value::from(3)])
        );
    }
    {
        let map = r.get(1).unwrap().as_map().unwrap();
        assert_eq!(
            map.get("payload").unwrap(),
            &Value::from(vec![Value::from(4), Value::from(5)])
        );
    }
}

async fn go(data: Value, size: usize) -> Vec<Value> {
    let mut service = FixedSize;

    let (tx, mut rx) = toy_core::mpsc::channel(10);
    let task_ctx = toy_plugin_test::dummy_task_context();

    let config = FixedSizeConfig::with(size);
    let mut c = service
        .new_context(toy_plugin_test::dummy_service_type(), config)
        .await
        .unwrap();

    // send...
    for v in data.as_vec().unwrap() {
        let r = service
            .handle(
                task_ctx.clone(),
                c,
                Frame::from_value(v.clone()),
                tx.clone(),
            )
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
    while let Some(item) = rx.next().await {
        result.push(item.into_value().unwrap());
    }

    result
}
