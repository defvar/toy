use toy_pack_derive::*;
use toy_pack_mp::{pack, unpack};

#[derive(Pack, Unpack, Debug, PartialEq)]
struct Dum<'a> {
    v_u32: u32,
    v_string: String,
    v_borrowed_str: &'a str,
}

#[derive(Eq, PartialEq, Debug, Pack, Unpack)]
enum TestEnum {
    //unit variant
    A,

    //newtype variant
    B(u32),

    //tuple variant
    C(u32, u32),
}

impl Default for TestEnum {
    fn default() -> Self {
        TestEnum::A
    }
}

#[derive(Debug, Pack, Unpack)]
struct Gen<T>
where
    T: std::default::Default
        + toy_pack::deser::DeserializableOwned<Value = T>
        + toy_pack::ser::Serializable,
{
    value: T,
}

#[derive(Pack)]
struct User {
    id: u32,
    name: String,
}

fn main() {
    let mut src: Vec<Dum> = Vec::new();
    let dum1 = Dum {
        v_u32: 1,
        v_string: "a".to_owned(),
        v_borrowed_str: "b",
    };

    let dum2 = Dum {
        v_u32: 2,
        v_string: "a".to_owned(),
        v_borrowed_str: "b",
    };

    src.push(dum1);
    src.push(dum2);

    let vec = pack(&src).unwrap();
    println!("{:?}", vec);

    let r = unpack::<Vec<Dum>>(vec.as_slice()).unwrap();
    println!("{:?}", r);

    println!("----------------------------------");

    let src = Gen::<u32> { value: 1 };
    let vec = pack(&src).unwrap();
    println!("{:?}", vec);

    let r = unpack::<Gen<u32>>(vec.as_slice()).unwrap();
    println!("{:?}", r);

    let src = User {
        id: 1,
        name: "a".to_string(),
    };
    let vec = pack(&src).unwrap();
    println!("{:?}", vec);
}
