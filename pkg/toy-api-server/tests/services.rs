use std::collections::HashMap;
use toy_api::services::ServiceSpec;
use toy_api_server::api::services;
use toy_api_server::authentication::NoAuth;
use toy_api_server::store::memory::MemoryStore;
use toy_core::prelude::PortType;
use toy_core::ServiceType;
use toy_h::NoopHttpClient;

mod util;

#[tokio::test]
async fn find() {
    util::prepare();

    let v = ServiceSpec::new(
        ServiceType::new("hoge", "moge").unwrap(),
        PortType::flow(),
        None,
    );

    let init = {
        let mut map = HashMap::new();
        map.insert(
            "/toy/services/hoge.moge".to_owned(),
            toy_core::data::pack(&v).unwrap(),
        );
        map
    };
    let store = MemoryStore::with_map(init);
    let filter = services(NoAuth, NoopHttpClient, store);

    let res = util::get().path("/services/hoge.moge").reply(&filter).await;

    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn put() {
    util::prepare();

    let v = ServiceSpec::new(
        ServiceType::new("hoge", "moge").unwrap(),
        PortType::flow(),
        None,
    );
    let body = toy_pack_json::pack_to_string(&v).unwrap();

    let store = MemoryStore::with_map(HashMap::new());
    let filter = services(NoAuth, NoopHttpClient, store);

    let res = util::put()
        .path("/services/hoge.moge")
        .body(body)
        .reply(&filter)
        .await;

    assert_eq!(res.status(), 201);

    let res = util::get().path("/services/hoge.moge").reply(&filter).await;

    assert_eq!(res.status(), 200);
}
