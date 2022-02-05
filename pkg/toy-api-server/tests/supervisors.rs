use chrono::Utc;
use std::collections::HashMap;
use toy_api::supervisors::Supervisor;
use toy_api_server::api::supervisors;
use toy_api_server::authentication::NoAuth;
use toy_api_server::store::memory::MemoryStore;
use toy_h::NoopHttpClient;

mod util;

#[tokio::test]
async fn find() {
    util::prepare();

    let v = Supervisor::new("aiueo".to_string(), Utc::now(), Vec::new());

    let init = {
        let mut map = HashMap::new();
        map.insert(
            "/toy/supervisors/aiueo".to_owned(),
            toy_core::data::pack(&v).unwrap(),
        );
        map
    };
    let store = MemoryStore::with_map(init);
    let filter = supervisors(NoAuth, NoopHttpClient, store);

    let res = util::get().path("/supervisors/aiueo").reply(&filter).await;

    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn put() {
    util::prepare();

    let v = Supervisor::new("aiueo".to_string(), Utc::now(), Vec::new());
    let body = toy_pack_json::pack_to_string(&v).unwrap();

    let store = MemoryStore::with_map(HashMap::new());
    let filter = supervisors(NoAuth, NoopHttpClient, store);

    let res = util::put()
        .path("/supervisors/aiueo")
        .body(body)
        .reply(&filter)
        .await;

    assert_eq!(res.status(), 201);

    let res = util::get().path("/supervisors/aiueo").reply(&filter).await;

    assert_eq!(res.status(), 200);
}
