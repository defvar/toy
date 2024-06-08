use serde::{Deserialize, Serialize};
use toy_core::data::Value;
use toy_plugin_filter::predicate::{Operator, Predicate};

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
struct TestPredicateHolder {
    p: Predicate,
}

#[test]
fn predicate_match() {
    let p = Predicate::new("hoge".to_string(), Operator::Match, "^abc");
    let cp = Value::String("abcdef".to_string());
    let r = p.is_match(&cp);
    assert_eq!(r, true);

    let cp = Value::String("fedcba".to_string());
    let r = p.is_match(&cp);
    assert_eq!(r, false);
}

#[test]
fn predicate_unmatch() {
    let p = Predicate::new("hoge".to_string(), Operator::Unmatch, "^abc");
    let cp = Value::String("fedcba".to_string());
    let r = p.is_match(&cp);
    assert_eq!(r, true);

    let cp = Value::String("abcdef".to_string());
    let r = p.is_match(&cp);
    assert_eq!(r, false);
}

#[test]
fn predicate_match_no_str() {
    let p = Predicate::new("hoge".to_string(), Operator::Match, "^abc");
    let cp = Value::Integer(1);
    let r = p.is_match(&cp);
    assert_eq!(r, false);
}
