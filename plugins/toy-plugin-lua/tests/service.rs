use toy_core::prelude::*;
use toy_plugin_lua::config::LuaFunctionConfig;
use toy_plugin_lua::service::LuaFunction;

#[tokio::test]
async fn test_lua_function() {
    let mut service = LuaFunction;
    let (tx, mut rx) = toy_core::mpsc::channel(10);
    let task_ctx = toy_plugin_test::dummy_task_context();
    let code = r#"
    toy.payload.message = "lua"
    toy.payload.number = 1
    "#;
    let config = LuaFunctionConfig {
        code: code.to_string(),
    };

    let c = service
        .new_context(toy_plugin_test::dummy_service_type(), config)
        .await
        .unwrap();

    let value = map_value! {
        "message" => "a",
        "number" => 0,
    };
    let expected = map_value! {
        "message" => "lua",
        "number" => 1,
    };
    let frame = Frame::from_value(value);

    let r = service.handle(task_ctx, c, frame, tx).await;
    assert!(r.is_ok());
    let r = rx.next().await.unwrap().value().cloned().unwrap();
    assert_eq!(r.path("message"), expected.path("message"));
    assert_eq!(
        r.path("number").unwrap().parse_integer::<i64>(),
        expected.path("number").unwrap().parse_integer::<i64>()
    );
}
