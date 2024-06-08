use toy_core::prelude::*;
use toy_plugin_filter::config::{FilterConfig};
use toy_plugin_filter::predicate::{Operator, Predicate};
use toy_plugin_filter::service::Filter;

#[tokio::test]
async fn filter() {
    toy_plugin_test::rust_log_debug();
    let _ = toy_plugin_test::tracing_console();

    let data = vec![
        map_value!("a" => 1, "b" => "9"),
        map_value!("a" => 1, "b" => "7"),
        map_value!("a" => 2, "b" => "8"),
    ];

    let preds = vec![
        Predicate::new("a", Operator::Eq, "1"),
        Predicate::new("b", Operator::LessThanOrEqual, "7"),
    ];

    let r = go(data, preds).await;

    {
        let map = r.get(0).unwrap().as_map().unwrap();
        // assert_eq!(
        //     map.get("payload").unwrap(),
        //     &Value::from(vec![Value::from(1), Value::from(2), Value::from(3)])
        // );
        println!("{:?}", map);
    }
}

async fn go(data: Vec<Value>, preds: Vec<Predicate>) -> Vec<Value> {
    let mut service = Filter;

    let (tx, mut rx) = toy_core::mpsc::channel(10);
    let task_ctx = toy_plugin_test::dummy_task_context();

    let config = FilterConfig::with(&preds);
    let mut c = service
        .new_context(toy_plugin_test::dummy_service_type(), config)
        .await
        .unwrap();

    // send...
    for v in data {
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
