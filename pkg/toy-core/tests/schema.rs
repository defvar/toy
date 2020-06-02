use std::collections::HashMap;
use toy_core::data::schema::visitors::*;
use toy_pack::{schema::to_schema, Schema};
use toy_pack_derive::*;

#[test]
fn aaaaa() {
    #[derive(Debug, Pack, Schema)]
    struct Dum {
        v_u8: u8,
        v_i8_opt: Option<i8>,
        v_f32: f32,
        v_f64: f64,
        name: String,
        vec: Vec<u32>,
        map: HashMap<String, String>,
        abc: ABC,
    };

    #[derive(Debug, Pack, Schema)]
    enum ABC {
        A,
        B(u32, u32),
        C,
    }

    let mut v = JsonSchemaVisitor {};
    let r = to_schema::<Dum, JsonSchemaVisitor>("aiueo!", &mut v).unwrap();
    let json = toy_pack_json::pack_to_string(&r).unwrap();
    println!("{:?}", json);
}
