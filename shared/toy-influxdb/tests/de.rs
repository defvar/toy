use chrono::{DateTime, Utc};
use serde::Deserialize;
use toy_influxdb::models::flux_table::{unpack, FluxTable, FluxTableBuilder};

#[test]
fn common_struct() {
    fn test_table() -> FluxTable {
        let mut builder = FluxTableBuilder::with_capacity(10, 10);
        builder.group(0, vec!["false"].into_iter());
        builder.data_type(
            0,
            vec![
                "boolean",
                "string",
                "double",
                "double",
                "unsignedLong",
                "long",
                "dateTime:RFC3339",
            ]
            .into_iter(),
        );
        builder.headers(
            0,
            vec![
                "flg",
                "name",
                "float_value_32",
                "float_value_64",
                "ulong_value_32",
                "long_value_32",
                "datetime_value",
            ]
            .into_iter(),
        );

        for i in 1..3 {
            builder.start_record(0, 0);
            builder.push(0, &format!("true")).unwrap();
            builder.push(1, &format!("name-{}", i)).unwrap();
            builder.push(2, &format!("{}", 3.2 * i as f32)).unwrap();
            builder.push(3, &format!("{}", 6.4 * i as f32)).unwrap();
            builder.push(4, &format!("{}", 32 * i)).unwrap();
            builder.push(5, &format!("{}", -32 * i)).unwrap();
            builder
                .push(6, &format!("2022-12-11T20:02:05.123456Z"))
                .unwrap();
            builder.end_record();
        }

        builder.build().get(0).unwrap().clone()
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    struct TestData {
        flg: bool,
        name: String,
        float_value_32: f32,
        float_value_64: f64,
        ulong_value_32: u32,
        long_value_32: i32,
        datetime_value: DateTime<Utc>,
    }

    let mut ft = test_table();
    //println!("{:?}", ft);
    let r = unpack::<Vec<TestData>>(&mut ft).unwrap();
    assert_eq!(
        r,
        vec![
            TestData {
                flg: true,
                name: "name-1".to_string(),
                float_value_32: 3.2,
                float_value_64: 6.4,
                ulong_value_32: 32,
                long_value_32: -32,
                datetime_value: DateTime::parse_from_rfc3339("2022-12-11T20:02:05.123456Z")
                    .unwrap()
                    .with_timezone(&Utc)
            },
            TestData {
                flg: true,
                name: "name-2".to_string(),
                float_value_32: 6.4,
                float_value_64: 12.8,
                ulong_value_32: 64,
                long_value_32: -64,
                datetime_value: DateTime::parse_from_rfc3339("2022-12-11T20:02:05.123456Z")
                    .unwrap()
                    .with_timezone(&Utc)
            }
        ]
    )
}

#[test]
fn unit_variant() {
    fn test_table() -> FluxTable {
        let mut builder = FluxTableBuilder::with_capacity(10, 10);
        builder.group(0, vec!["false"].into_iter());
        builder.data_type(0, vec!["string"].into_iter());
        builder.headers(0, vec!["Test"].into_iter());

        for _ in 0..10 {
            builder.start_record(0, 0);
            builder.push(0, "B").unwrap();
            builder.end_record();
        }

        builder.build().get(0).unwrap().clone()
    }

    #[derive(Debug, PartialEq, Deserialize)]
    enum Test {
        A,
        B,
    }
    let mut ft = test_table();
    let r = unpack::<Test>(&mut ft).unwrap();
    assert_eq!(r, Test::B);
}
