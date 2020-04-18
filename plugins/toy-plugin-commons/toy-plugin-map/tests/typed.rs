use std::collections::HashMap;
use toy_core::prelude::*;
use toy_plugin_map::config::{TypedConfig, TypedConfigOption};
use toy_plugin_map::{convert, AllowedTypes};

#[test]
fn typed() {
    let mut actual = map_value! {
      "a" => "2",
      "b" => "4",
    };

    let expected = map_value! {
      "a" => 2u8,
      "b" => 4u32,
    };

    let config = {
        let mut map = HashMap::new();
        map.insert(
            "a".to_string(),
            TypedConfigOption {
                tp: AllowedTypes::U8,
                default_value: None,
            },
        );
        map.insert(
            "b".to_string(),
            TypedConfigOption {
                tp: AllowedTypes::U32,
                default_value: None,
            },
        );
        map.insert(
            "xxxxxxx".to_string(),
            TypedConfigOption {
                tp: AllowedTypes::U32,
                default_value: None,
            },
        );
        TypedConfig { typed: map }
    };
    convert(&mut actual, &config);
    assert_eq!(actual, expected);
}

#[test]
fn typed_default_value() {
    let mut actual = map_value! {
      "a" => "xxxxx",
    };

    let expected = map_value! {
      "a" => 0u8,
    };

    let config = {
        let mut map = HashMap::new();
        map.insert(
            "a".to_string(),
            TypedConfigOption {
                tp: AllowedTypes::U8,
                default_value: Some("0".to_string()),
            },
        );
        TypedConfig { typed: map }
    };
    convert(&mut actual, &config);
    assert_eq!(actual, expected);
}
