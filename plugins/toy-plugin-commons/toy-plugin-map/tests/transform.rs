use std::collections::HashMap;
use toy_core::prelude::*;
use toy_plugin_map::config::{
    IndexedConfig, NamedConfig, PutConfig, RenameConfig, ReorderConfig, ToTransform,
};
use toy_plugin_map::{PutValue, Transformer};

#[test]
fn named() {
    let mut target = seq_value![0, 1, 2, 3, 4, 5];

    let expected = map_value! {
      "a" => 2,
      "b" => 4,
      "c" => 5,
    };

    let named = {
        let mut map = HashMap::new();
        map.insert("a".to_string(), 2);
        map.insert("b".to_string(), 4);
        map.insert("c".to_string(), 5);
        map
    };

    NamedConfig { named: Some(named) }
        .into_transform()
        .unwrap()
        .transform(&mut target)
        .unwrap();
    assert_eq!(target, expected);
}

#[test]
fn indexed() {
    let mut target = map_value! {
      "a" => 1,
      "b" => 2,
      "c" => map_value! {
        "A" => 31,
        "B" => 32,
      },
    };

    let expected = seq_value![31, 2, 1];

    let indexed = vec!["c.A".to_string(), "b".to_string(), "a".to_string()];

    IndexedConfig {
        indexed: Some(indexed),
    }
    .into_transform()
    .unwrap()
    .transform(&mut target)
    .unwrap();
    assert_eq!(target, expected);
}

#[test]
fn reorder() {
    let mut target = seq_value![3, 1, 2];
    let expected = seq_value![1, 2, 3];

    ReorderConfig {
        reorder: Some(vec![1, 2, 0]),
    }
    .into_transform()
    .unwrap()
    .transform(&mut target)
    .unwrap();
    assert_eq!(target, expected);
}

#[test]
fn rename() {
    let mut target = map_value! {
        "a" => 1,
        "b" => 2,
        "c" => 3
    };
    let expected = map_value! {
        "A" => 1,
        "B" => 2,
        "c" => 3
    };

    let rename = {
        let mut map = HashMap::new();
        map.insert("a".to_string(), "A".to_string());
        map.insert("b".to_string(), "B".to_string());
        map.insert("x".to_string(), "xxx".to_string());
        map
    };

    RenameConfig {
        rename: Some(rename),
    }
    .into_transform()
    .unwrap()
    .transform(&mut target)
    .unwrap();
    assert_eq!(target, expected);
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
    PutConfig { put: Some(put) }
        .into_transform()
        .unwrap()
        .transform(&mut target)
        .unwrap();
    assert_eq!(target, expected);
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
    PutConfig { put: Some(put) }
        .into_transform()
        .unwrap()
        .transform(&mut target)
        .unwrap();
    assert_eq!(target, expected);
}
