use toy_file::Row;

#[test]
fn empty() {
    let r = Row::new();
    assert_eq!(0, r.len());
    assert_eq!(None, r.get(0));
}

#[test]
fn push() {
    let mut r = Row::new();
    r.push(b"a");
    r.push(b"b");
    assert_eq!(2, r.len());
    assert_eq!(Some("a".as_bytes()), r.get(0));
    assert_eq!(Some("b".as_bytes()), r.get(1));
    assert_eq!(None, r.get(2));
}

#[test]
fn iter() {
    let data = vec!["a", "b", "c"];
    let mut r = Row::new();
    r.push(b"a");
    r.push(b"b");
    r.push(b"c");

    let actual = r.iter().map(|c| std::str::from_utf8(c).unwrap()).collect::<Vec<_>>();
    assert_eq!(data, actual);
}

#[test]
fn iter_rev() {
    let mut data = vec!["a", "b", "c"];
    let mut r = Row::new();
    r.push(b"a");
    r.push(b"b");
    r.push(b"c");

    let actual = r
        .iter()
        .rev()
        .map(|c| std::str::from_utf8(c).unwrap())
        .collect::<Vec<_>>();
    data.reverse();
    assert_eq!(data, actual);
}
