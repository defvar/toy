use chrono::{NaiveDate, Utc};
use toy_influxdb::models::line_protocol::{
    Field, FieldSet, LineProtocolRecord, Tag, TagSet, ToLineProtocol,
};
use toy_influxdb::models::FieldValue;

#[test]
fn tag() {
    let tag = Tag::with("a", "b");
    let mut buf = Vec::new();
    let r = tag.to_lp(&mut buf).unwrap();
    assert_eq!(r, 3);
    assert_eq!(std::str::from_utf8(&buf).unwrap(), "a=b");
}

#[test]
fn tags() {
    let vec = vec![Tag::with("a", "123"), Tag::with("b", "456")];
    let tags = TagSet::with(vec.into_iter());
    let mut buf = Vec::new();
    let r = tags.to_lp(&mut buf).unwrap();
    assert_eq!(r, 11);
    assert_eq!(std::str::from_utf8(&buf).unwrap(), "a=123,b=456");
}

#[test]
fn field() {
    let field = Field::with("a", FieldValue::Integer(123));
    let mut buf = Vec::new();
    let r = field.to_lp(&mut buf).unwrap();
    assert_eq!(r, 5);
    assert_eq!(std::str::from_utf8(&buf).unwrap(), "a=123");
}

#[test]
fn fields() {
    let vec = vec![
        Field::with("a", FieldValue::Integer(123)),
        Field::with("b", FieldValue::Integer(456)),
    ];
    let fields = FieldSet::with(vec.into_iter());
    let mut buf = Vec::new();
    let r = fields.to_lp(&mut buf).unwrap();
    assert_eq!(r, 11);
    assert_eq!(std::str::from_utf8(&buf).unwrap(), "a=123,b=456");
}

#[test]
fn record() {
    let vec = vec![Tag::with("a", "123"), Tag::with("b", "456")];
    let tags = TagSet::with(vec.into_iter());
    let vec = vec![
        Field::with("field_a", FieldValue::Integer(123)),
        Field::with("field_b", FieldValue::String("hoge".to_string())),
    ];
    let fields = FieldSet::with(vec.into_iter());

    let dt = NaiveDate::from_ymd_opt(2001, 9, 9)
        .unwrap()
        .and_hms_nano_opt(1, 46, 40, 555)
        .unwrap()
        .and_local_timezone(Utc)
        .unwrap();
    assert_eq!(dt.timestamp_nanos(), 1_000_000_000_000_000_555);

    let record = LineProtocolRecord::with("test", tags, fields, dt);

    let mut buf = Vec::new();
    record.to_lp(&mut buf).unwrap();
    assert_eq!(
        std::str::from_utf8(&buf).unwrap(),
        "test,a=123,b=456 field_a=123,field_b=\"hoge\" 1000000000000000555"
    );
}
