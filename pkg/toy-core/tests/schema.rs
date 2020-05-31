use std::collections::HashMap;
use toy_core::data::schema::visitors::*;
use toy_pack::schema::*;
use toy_pack_derive::*;

#[test]
fn aaaaa() {
    #[derive(Debug, Pack)]
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

    #[derive(Debug, Pack)]
    enum ABC {
        A,
        B(u32, u32),
        C,
    }

    impl Schema for ABC {
        fn scan<V>(name: &'static str, visitor: &mut V) -> Result<V::Value, V::Error>
        where
            V: SchemaVisitor,
        {
            let mut v = visitor.enum_visitor(name, "ABC")?;
            v.unit_variant("ABC", "A")?;
            let b = {
                let mut tuple_visitor = v.tuple_variant_visitor("ABC", "B")?;
                tuple_visitor.tuple_variant_arg::<u32>(name, "B", 0)?;
                tuple_visitor.tuple_variant_arg::<u32>(name, "B", 1)?;
                tuple_visitor.end()?
            };
            v.variant("ABC", "B", b)?;
            v.unit_variant("ABC", "C")?;
            v.end()
        }
    }

    impl Schema for Dum {
        fn scan<V>(_name: &'static str, visitor: &mut V) -> Result<V::Value, V::Error>
        where
            V: SchemaVisitor,
        {
            let mut v = visitor.struct_visitor("Dum")?;
            v.field::<u8>("v_u8")?;
            v.field::<Option<i8>>("v_i8_opt")?;
            v.field::<ABC>("abc")?;
            v.end()
        }
    }

    let mut v = JsonSchemaVisitor {};
    let r = Dum::scan("aiueo!", &mut v).unwrap();
    let json = toy_pack_json::pack_to_string(&r).unwrap();
    println!("{:?}", json);
}
