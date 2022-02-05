use std::collections::HashMap;
use toy_api::graph::{Graph, GraphNode, Position};
use toy_api_server::api::graphs;
use toy_api_server::authentication::NoAuth;
use toy_api_server::store::memory::MemoryStore;
use toy_core::data::Value;
use toy_h::NoopHttpClient;

mod util;

#[tokio::test]
async fn find() {
    util::prepare();

    let n = GraphNode::new(
        "hoge.moge",
        "s1",
        Position::default(),
        None,
        Value::None,
        Vec::new(),
    );
    let v = Graph::new("hoge", vec![n]);

    let init = {
        let mut map = HashMap::new();
        map.insert(
            "/toy/graphs/hoge".to_owned(),
            toy_core::data::pack(&v).unwrap(),
        );
        map
    };
    let store = MemoryStore::with_map(init);
    let filter = graphs(NoAuth, NoopHttpClient, store);

    let res = util::get().path("/graphs/hoge").reply(&filter).await;

    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn put() {
    util::prepare();

    let n = GraphNode::new(
        "hoge.moge",
        "s1",
        Position::default(),
        None,
        Value::None,
        Vec::new(),
    );
    let v = Graph::new("hoge", vec![n]);
    let body = toy_pack_json::pack_to_string(&v).unwrap();

    let store = MemoryStore::with_map(HashMap::new());
    let filter = graphs(NoAuth, NoopHttpClient, store);

    let res = util::put()
        .path("/graphs/hoge")
        .body(body)
        .reply(&filter)
        .await;

    assert_eq!(res.status(), 201);

    let res = util::get().path("/graphs/hoge").reply(&filter).await;

    assert_eq!(res.status(), 200);
}
