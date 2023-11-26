mod util;

#[test]
fn put_and_get() {
    let c = util::setup("put_and_get");
    c.put(b"a", b"111").unwrap();

    let r = c.get("a").unwrap().unwrap();
    assert_eq!(r, b"111");
}

#[test]
fn iterator() {
    let c = util::setup("iterator");
    let data = [
        (b"a", b"111"),
        (b"b", b"222"),
        (b"c", b"333"),
        (b"d", b"444"),
    ];
    c.put_batch(&data).unwrap();

    let r = c.iter().unwrap();

    for (idx, r) in r.enumerate() {
        let (k, v) = r.unwrap();
        assert_eq!(k.as_ref(), &data[idx].0[..]);
        assert_eq!(v.as_ref(), &data[idx].1[..]);
    }
}
