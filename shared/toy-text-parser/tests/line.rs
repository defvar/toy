use toy_text_parser::Line;

#[test]
fn empty() {
    let r = Line::new();
    assert_eq!(0, r.len());
    assert_eq!(None, r.get(0));
}

#[test]
fn get_len() {
    let mut r = Line::new();

    assert_eq!(0, r.len());
    assert_eq!(0, r.len_bytes());

    r.push(b"abc");

    assert_eq!(1, r.len());
    assert_eq!(3, r.len_bytes());
}

#[test]
fn push() {
    let mut r = Line::new();
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
    let mut r = Line::new();
    r.push(b"a");
    r.push(b"b");
    r.push(b"c");

    let actual = r
        .iter()
        .map(|c| std::str::from_utf8(c).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(data, actual);
}

#[test]
fn iter_rev() {
    let mut data = vec!["a", "b", "c"];
    let mut r = Line::new();
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
