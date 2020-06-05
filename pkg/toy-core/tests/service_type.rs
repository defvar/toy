use toy_core::ServiceType;

#[test]
fn st_from_str() {
    assert_eq!(
        ServiceType::from_full_name("a.b.c").unwrap(),
        ServiceType::new("a.b", "c").unwrap()
    );
}
