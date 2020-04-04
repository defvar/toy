use std::collections::HashMap;
use toy_core::prelude::*;
use toy_plugin_map::config::{IndexedConfig, NamedConfig, PutConfig, ReorderConfig, ToTransform};
use toy_plugin_map::PutValue;

#[test]
fn named() {
    let mut target = seq_value![0, 1, 2, 3, 4, 5];

    let expected = map_value! {
      "a" => 2,
      "b" => 4,
      "c" => 5,
    };

    let mut map = HashMap::new();
    map.insert("a".to_string(), 2);
    map.insert("b".to_string(), 4);
    map.insert("c".to_string(), 5);

    let actual = NamedConfig { named: Some(map) }
        .into_transform()
        .unwrap()
        .transform(&mut target)
        .unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn indexed() {
    let mut target = map_value! {
      "a" => 2,
      "b" => 4,
      "c" => 5,
    };

    let expected = seq_value![5, 4, 2];

    let indexed = vec!["c".to_string(), "b".to_string(), "a".to_string()];

    let actual = IndexedConfig {
        indexed: Some(indexed),
    }
    .into_transform()
    .unwrap()
    .transform(&mut target)
    .unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn reorder() {
    let mut target = seq_value![3, 1, 2];
    let expected = seq_value![1, 2, 3];

    let actual = ReorderConfig {
        reorder: Some(vec![1, 2, 0]),
    }
    .into_transform()
    .unwrap()
    .transform(&mut target)
    .unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn put_map() {
    let mut target = map_value! {
      "a" => 2u32,
    };
    let expected = map_value! {
      "a" => 2u32,
      "b" => 4u32,
    };
    let put = {
        let mut map = HashMap::new();
        map.insert("b".to_string(), PutValue::new(Some("4".to_string()), "u32"));
        map
    };
    let actual = PutConfig { put: Some(put) }
        .into_transform()
        .unwrap()
        .transform(&mut target)
        .unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn put_map_err() {
    let mut target = map_value! {
      "a" => 2u32,
    };
    let expected = map_value! {
      "a" => 2u32,
      "b" => Value::None,
    };
    let put = {
        let mut map = HashMap::new();
        map.insert(
            "b".to_string(),
            PutValue::new(Some("4".to_string()), "xxxx"),
        );
        map
    };
    let actual = PutConfig { put: Some(put) }
        .into_transform()
        .unwrap()
        .transform(&mut target)
        .unwrap();
    assert_eq!(actual, expected);
}
