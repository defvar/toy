use toy_core::data::{Map, Value};
use toy_plugin_file::{FileWriter, FileWriterBuilder};

#[test]
fn write_value() {
    let vec: Vec<u8> = Vec::new();
    let mut w = FileWriterBuilder::default().from_writer(vec);

    let mut two_map = Map::new();
    two_map.insert("a".to_string(), Value::from(21u32));
    two_map.insert("b".to_string(), Value::from(22u32));

    let three_seq = vec![Value::from(31), Value::from(32)];

    let mut map = Map::new();
    map.insert("one".to_string(), Value::from(1u32));
    map.insert("two".to_string(), Value::from(two_map));
    map.insert("three".to_string(), Value::from(three_seq));

    let v = Value::from(map);
    let _ = w.write_value(&v);
    let _ = w.flush();
    assert_eq!(
        w_to_string(w),
        "one,two.a,two.b,three.0,three.1\r\n1,21,22,31,32\r\n"
    )
}

fn w_to_string(w: FileWriter<Vec<u8>>) -> String {
    String::from_utf8(w.into_inner().unwrap()).unwrap()
}
