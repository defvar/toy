use chrono::Utc;
use toy_influxdb::models::FieldValue;
use toy_influxdb::query::builder::{Filter, FluxBuilder, From, Range};

#[test]
fn range() {
    let now = Utc::now();
    let r = FluxBuilder::default()
        .push(From::with("toy"))
        .push(Range::start(now))
        .to_flux()
        .unwrap();

    assert_eq!(
        r,
        format!(
            "from(bucket: \"toy\") |> range(start: {})",
            now.to_rfc3339()
        )
    );
}

#[test]
fn filter() {
    let r = FluxBuilder::from("toy")
        .push(Filter::eq("a", FieldValue::String("hoge".to_string())))
        .to_flux()
        .unwrap();

    assert_eq!(
        r,
        format!(
            "from(bucket: \"toy\") |> filter(fn: (r) => r.a == {})",
            "\"hoge\""
        )
    );
}

#[test]
fn filter_regex() {
    let r = FluxBuilder::from("toy")
        .push(Filter::regex_match(
            "a",
            FieldValue::String("hoge".to_string()),
        ))
        .to_flux()
        .unwrap();

    assert_eq!(
        r,
        format!(
            "from(bucket: \"toy\") |> filter(fn: (r) => r.a =~ {})",
            "/hoge/"
        )
    );
}

#[test]
fn ungroup() {
    let r = FluxBuilder::from("toy").ungroup().to_flux().unwrap();

    assert_eq!(r, "from(bucket: \"toy\") |> group()");
}

#[test]
fn group() {
    let r = FluxBuilder::from("toy")
        .group(&["a", "b"])
        .to_flux()
        .unwrap();

    assert_eq!(r, "from(bucket: \"toy\") |> group(columns: [\"a\",\"b\"])");
}

#[test]
fn drop() {
    let r = FluxBuilder::from("toy")
        .drop(&["a", "b"])
        .to_flux()
        .unwrap();

    assert_eq!(r, "from(bucket: \"toy\") |> drop(columns: [\"a\",\"b\"])");
}

#[test]
fn pivot() {
    let r = FluxBuilder::from("toy")
        .pivot(&["a", "b"], &["_field"], "_value")
        .to_flux()
        .unwrap();

    assert_eq!(
        r,
        "from(bucket: \"toy\") |> pivot(rowKey: [\"a\",\"b\"], columnKey: [\"_field\"], valueColumn: \"_value\")"
    );
}

#[test]
fn rename() {
    let r = FluxBuilder::from("toy")
        .rename(&[("_a", "a"), ("_b", "b")])
        .to_flux()
        .unwrap();

    assert_eq!(
        r,
        "from(bucket: \"toy\") |> rename(columns: {_a:\"a\", _b:\"b\"})"
    );
}

#[test]
fn sort() {
    let r = FluxBuilder::from("toy")
        .sort(&["a", "b"])
        .to_flux()
        .unwrap();

    assert_eq!(r, "from(bucket: \"toy\") |> sort(columns: [\"a\",\"b\"])");
}

#[test]
fn limit() {
    let r = FluxBuilder::from("toy").limit(5).to_flux().unwrap();

    assert_eq!(r, "from(bucket: \"toy\") |> limit(n: 5)");
}

#[test]
fn limit_offset() {
    let r = FluxBuilder::from("toy")
        .limit_and_offset(100, 5)
        .to_flux()
        .unwrap();

    assert_eq!(r, "from(bucket: \"toy\") |> limit(n: 100, offset: 5)");
}
