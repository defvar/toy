use std::io::BufReader;
use toy_log_collector::LineReader;
use toy_text_parser::dfa::ByteParserBuilder;
use toy_text_parser::Line;

#[test]
fn read_line() {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.push(b'1');
    bytes.push(b'2');
    bytes.push(b'\r');
    bytes.push(b'3');

    let p = ByteParserBuilder::default().build();
    let mut r = LineReader::new(p, BufReader::new(&*bytes));
    let mut line = Line::new();
    assert_eq!(r.read(&mut line).unwrap(), true);
    assert_eq!(to_str(&mut line), "12");
    line.clear();
    assert_eq!(r.read(&mut line).unwrap(), true);
    assert_eq!(to_str(&mut line), "3");

    assert_eq!(r.read(&mut line).unwrap(), false);
}

fn to_str(line: &mut Line) -> &str {
    std::str::from_utf8(line.get(0).unwrap()).unwrap()
}
