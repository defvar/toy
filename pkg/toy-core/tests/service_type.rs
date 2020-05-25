use toy_core::ServiceType;

#[test]
fn st_from_str() {
    assert_eq!(ServiceType::from("a.b.c"), ServiceType::new("a.b", "c"));
}
