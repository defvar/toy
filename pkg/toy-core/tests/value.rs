use toy_core::data::Value;

#[test]
fn parse_str_u64() {
    assert_eq!(Value::from(1u64).parse_str(), Some(Value::from("1")))
}

#[test]
fn parse_number_u64() {
    assert_eq!(
        Value::from("123").parse_number::<u64>(),
        Some(Value::from(123u64))
    )
}

#[test]
fn parse_number_f32() {
    assert_eq!(
        Value::from("123").parse_number::<f32>(),
        Some(Value::from(123f32))
    )
}

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
