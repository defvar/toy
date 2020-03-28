use toy_core::data::Value;

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
