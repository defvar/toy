use toy_core::ServiceType;

#[test]
fn st_from_str() {
    assert_eq!(
        ServiceType::from_full_name("a.b.c").unwrap(),
        ServiceType::new("a.b", "c").unwrap()
    );
}

#[test]
fn ser() {
    assert_eq!(
        std::str::from_utf8(
            &toy_pack_json::pack(&ServiceType::from_full_name("a.b.c").unwrap()).unwrap()
        )
        .unwrap(),
        "\"a.b.c\""
    );
}

#[test]
fn de() {
    assert_eq!(
        toy_pack_json::unpack::<ServiceType>("\"a.b.c\"".as_bytes()).unwrap(),
        ServiceType::from_full_name("a.b.c").unwrap()
    );
}
