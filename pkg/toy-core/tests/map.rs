use toy_core::data::Map;

#[test]
fn get_or_insert() {
    let mut map = Map::new();
    map.insert("a", 1);

    assert_eq!(map.get_or_insert("a", 2222), &mut 1);
    assert_eq!(map.get_or_insert("b", 2), &mut 2);
}
