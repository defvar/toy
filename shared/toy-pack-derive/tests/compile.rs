use toy_pack_derive::*;

#[test]
fn common_struct() {
    let data = Data {
        v_u32: 1,
        v_string: "a".to_owned(),
        v_borrowed_str: "b",
        v_struct: Inner { v_u32: 11 },
        v_vec: vec![Inner { v_u32: 121 }, Inner { v_u32: 122 }],
        v_vec_empty: vec![],
        v_enum: TestEnum::C(456, 789),
    };

    let expected = DataUnpack {
        v_u32: 1,
        v_string: "a".to_owned(),
        v_borrowed_str: "b",
        v_struct: Inner { v_u32: 11 },
        v_vec: vec![Inner { v_u32: 121 }, Inner { v_u32: 122 }],
        v_vec_empty: vec![],
        v_enum: TestEnum::C(456, 789),
        v_default: u32::default(),
        v_default_u32: 999,
        v_default_bool: true,
        v_default_byte: b',',
        v_default_expr: default_resource(),
    };

    let vec = toy_pack_mp::pack(&data).unwrap();
    let actual = toy_pack_mp::unpack::<DataUnpack>(vec.as_slice()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn unit_variant() {
    let vec = toy_pack_mp::pack(&TestEnum::A).unwrap();
    let actual = toy_pack_mp::unpack::<TestEnum>(vec.as_slice()).unwrap();

    assert_eq!(TestEnum::A, actual);
}

#[test]
fn newtype_variant() {
    let v = TestEnum::B(123);
    let vec = toy_pack_mp::pack(&v).unwrap();
    let actual = toy_pack_mp::unpack::<TestEnum>(vec.as_slice()).unwrap();

    assert_eq!(v, actual);
}

#[test]
fn tuple_variant() {
    let v = TestEnum::C(123, 456);
    let vec = toy_pack_mp::pack(&v).unwrap();
    let actual = toy_pack_mp::unpack::<TestEnum>(vec.as_slice()).unwrap();

    assert_eq!(v, actual);
}

#[derive(Pack, Debug, PartialEq)]
struct Data<'a> {
    v_u32: u32,
    v_string: String,
    v_borrowed_str: &'a str,
    v_struct: Inner,
    v_vec: Vec<Inner>,
    v_vec_empty: Vec<Inner>,
    v_enum: TestEnum,
}

#[derive(UnPack, Debug, PartialEq)]
struct DataUnpack<'a> {
    v_u32: u32,
    v_string: String,
    v_borrowed_str: &'a str,
    v_struct: Inner,
    v_vec: Vec<Inner>,
    v_vec_empty: Vec<Inner>,
    v_enum: TestEnum,

    #[toy(default)]
    v_default: u32,
    #[toy(default = 999u32)]
    v_default_u32: u32,
    #[toy(default = true)]
    v_default_bool: bool,
    #[toy(default = b',')]
    v_default_byte: u8,
    #[toy(default_expr = "default_resource")]
    v_default_expr: String,
}

fn default_resource() -> String {
    "/".to_string()
}

#[derive(Pack, UnPack, Debug, PartialEq)]
struct Inner {
    v_u32: u32,
}

impl Default for Inner {
    fn default() -> Self {
        Self { v_u32: 0 }
    }
}

#[derive(Eq, PartialEq, Debug, Pack, UnPack)]
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
