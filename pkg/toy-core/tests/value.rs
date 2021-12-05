use toy_core::data::Value;
use toy_core::prelude::*;

#[test]
fn path() {
    let v = map_value! {
        "a" => 1,
        "b" => 2,
        "c" => map_value! {
            "ca" => 31,
            "cb" => 32,
        },
        "d" => seq_value![41,42,43]
    };

    assert_eq!(v.path("xxx"), None);
    assert_eq!(v.path("a").unwrap(), &Value::from(1));
    assert_eq!(v.path("c.ca").unwrap(), &Value::from(31));
    assert_eq!(v.path("d.2").unwrap(), &Value::from(43));
}

#[test]
fn insert_by_path() {
    let mut v = map_value! {
        "a" => 1,
    };

    let expected_1 = map_value! {
        "a" => 1,
        "b" => 2,
    };

    let expected_2 = map_value! {
        "a" => 1,
        "b" => 2,
        "c" => map_value! {
            "ca" => 31,
        },
    };

    assert_eq!(v.insert_by_path("b", Value::from(2)), None);
    assert_eq!(v, expected_1);
    assert_eq!(v.insert_by_path("c.ca", Value::from(31)), None);
    assert_eq!(v, expected_2);
}

#[test]
fn insert_by_path_overwrite() {
    let mut v = Value::from(1);

    let expected_1 = map_value! {
        "a" => 1,
    };

    assert_eq!(v.insert_by_path("a", Value::from(1)), None);
    assert_eq!(v, expected_1);
    assert_eq!(
        v.insert_by_path("a", Value::from(2)).unwrap(),
        Value::from(1)
    );
}

macro_rules! pass_parse_integer {
    ($func: ident, $t: ident, $actual: expr, $expected: expr) => {
        #[test]
        fn $func() {
            assert_eq!(Value::from($actual).parse_integer::<$t>(), Some($expected))
        }
    };
}

macro_rules! none_parse_integer {
    ($func: ident, $t: ident, $actual: expr) => {
        #[test]
        fn $func() {
            assert_eq!(Value::from($actual).parse_integer::<$t>(), None)
        }
    };
}

///////////////////////////////////
// u8 /////////////////////////////
///////////////////////////////////
pass_parse_integer!(parse_u8_from_u8_0, u8, 0u8, 0u8);
pass_parse_integer!(parse_u8_from_u8_max, u8, u8::max_value(), u8::max_value());

pass_parse_integer!(parse_u8_from_u16_0, u8, 0u16, 0u8);
none_parse_integer!(parse_u8_from_u16_max, u8, u16::max_value());

pass_parse_integer!(parse_u8_from_u32_0, u8, 0u32, 0u8);
none_parse_integer!(parse_u8_from_u32_max, u8, u32::max_value());

pass_parse_integer!(parse_u8_from_u64_0, u8, 0u64, 0u8);
none_parse_integer!(parse_u8_from_u64_max, u8, u64::max_value());

////////////////////////////////////
// u16 /////////////////////////////
////////////////////////////////////
pass_parse_integer!(parse_u16_from_u8_0, u16, 0u8, 0u16);
pass_parse_integer!(
    parse_u16_from_u8_max,
    u16,
    u8::max_value(),
    u8::max_value() as u16
);

pass_parse_integer!(parse_u16_from_u16_0, u16, 0u16, 0u16);
pass_parse_integer!(
    parse_u16_from_u16_max,
    u16,
    u16::max_value(),
    u16::max_value()
);

pass_parse_integer!(parse_u16_from_u32_0, u16, 0u32, 0u16);
none_parse_integer!(parse_u16_from_u32_max, u16, u32::max_value());

pass_parse_integer!(parse_u16_from_u64_0, u16, 0u64, 0u16);
none_parse_integer!(parse_u16_from_u64_max, u16, u64::max_value());

////////////////////////////////////
// u32 /////////////////////////////
////////////////////////////////////
pass_parse_integer!(parse_u32_from_u8_0, u32, 0u8, 0u32);
pass_parse_integer!(
    parse_u32_from_u8_max,
    u32,
    u8::max_value(),
    u8::max_value() as u32
);

pass_parse_integer!(parse_u32_from_u16_0, u32, 0u16, 0u32);
pass_parse_integer!(
    parse_u32_from_u16_max,
    u32,
    u16::max_value(),
    u16::max_value() as u32
);

pass_parse_integer!(parse_u32_from_u32_0, u32, 0u32, 0u32);
pass_parse_integer!(
    parse_u32_from_u32_max,
    u32,
    u32::max_value(),
    u32::max_value()
);

pass_parse_integer!(parse_u32_from_u64_0, u32, 0u64, 0u32);
none_parse_integer!(parse_u32_from_u64_max, u32, u64::max_value());

////////////////////////////////////
// u64 /////////////////////////////
////////////////////////////////////
pass_parse_integer!(parse_u64_from_u8_0, u64, 0u8, 0u64);
pass_parse_integer!(
    parse_u64_from_u8_max,
    u64,
    u8::max_value(),
    u8::max_value() as u64
);

pass_parse_integer!(parse_u64_from_u16_0, u64, 0u16, 0u64);
pass_parse_integer!(
    parse_u64_from_u16_max,
    u64,
    u16::max_value(),
    u16::max_value() as u64
);

pass_parse_integer!(parse_u64_from_u32_0, u64, 0u32, 0u64);
pass_parse_integer!(
    parse_u64_from_u32_max,
    u64,
    u32::max_value(),
    u32::max_value() as u64
);
none_parse_integer!(parse_u64_from_u64_max, u64, u64::max_value());

#[test]
fn partial_eq_u64() {
    let mut v = Value::from(1u64);
    let other = 1u64;
    assert_eq!(&v, other);
    assert_eq!(&mut v, other);
    assert_eq!(v, other);
    assert_eq!(other, v);
}

#[test]
fn partial_eq_f64() {
    let mut v = Value::from(1f64);
    let other = 1f64;
    assert_eq!(&v, other);
    assert_eq!(&mut v, other);
    assert_eq!(v, other);
    assert_eq!(other, v);
}

#[test]
fn partial_eq_bool() {
    let mut v = Value::from(true);
    let other = true;
    assert_eq!(&v, other);
    assert_eq!(&mut v, other);
    assert_eq!(v, other);
    assert_eq!(other, v);
}

#[test]
fn partial_eq_str() {
    let mut v = Value::from("aiueo");
    let other = "aiueo";
    assert_eq!(&v, other);
    assert_eq!(&mut v, other);
    assert_eq!(v, other);
    assert_eq!(other, v);
}

#[test]
fn partial_eq_string() {
    let v = Value::from("aiueo");
    let other = "aiueo".to_string();
    assert_eq!(v, other);
    assert_eq!(other, v);
}

#[test]
fn ord_integer() {
    let small = Value::Integer(1);
    let big = Value::Integer(2);
    assert_eq!(small < big, true);
}

#[test]
fn ord_integer_number() {
    let small = Value::Integer(1);
    let big = Value::Number(1.1f64);
    assert_eq!(small < big, true);
}

#[test]
fn ord_integer_string() {
    let small = Value::Integer(2);
    let big = Value::String("1".to_string());
    assert_eq!(small < big, true);
}
