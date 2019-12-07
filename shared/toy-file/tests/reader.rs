use toy_file::parse::{ReaderBuilder, ReadResult};

macro_rules! exp {
  ($([$($field:expr),*]),*) => {{
     #[allow(unused_mut)]
     fn x() -> Vec<Row> {
         let mut csv: Vec<Row> = Vec::with_capacity(1024);
         $(
            let mut row = Row::new();
            $(
               row.push($field);
             )*
             csv.push(row);
          )*
          csv
     }
     x()
  }}
}

macro_rules! parse_test {
    ($name: ident, $data: expr, $expected: expr) => {
        #[test]
        fn $name() {
            let actual = parse($data);
            assert_eq!($expected, actual);
        }
    };
}

parse_test!(one_row_no_crlf, b"123", exp![["123"]]);
parse_test!(one_row_crlf, b"123\r\n", exp![["123"]]);
parse_test!(one_row_lf, b"123\r", exp![["123"]]);

parse_test!(one_row_trailing_delimiter, b"123,abc,\r", exp![["123", "abc", ""]]);

parse_test!(
    one_row_field_quoted,
    b"123,\"efg\",\"ijk\"",
    exp![["123", "efg", "ijk"]]
);
parse_test!(one_row_field_crlf, b"123,\"ef\r\ng\"", exp![["123", "ef\r\ng"]]);

parse_test!(two_row_no_crlf, b"123\r456", exp![["123"], ["456"]]);
parse_test!(two_row_crlf, b"123\r\n456\r\n", exp![["123"], ["456"]]);
parse_test!(two_row_lf, b"123\r456\r", exp![["123"], ["456"]]);

parse_test!(basic_lf, b"123,abc\r456,def\r", exp![["123", "abc"], ["456", "def"]]);

parse_test!(empty, b"", exp![]);
parse_test!(only_terminator, b"\r\n\r\n", exp![]);

parse_test!(empty_line, b"1,2\r\r3,4", exp![["1", "2"], ["3", "4"]]);

#[test]
fn column_edges() {
    let mut r = ReaderBuilder::default().build();
    let text = b"ai,ue\r";
    let data = &text[..];
    let mut buf = [0u8; 1024];
    let mut idx = [0usize; 10];
    let (_, _, _, _) = r.read_record(data, &mut buf, &mut idx);

    assert_eq!(2, idx[0]);
    assert_eq!(4, idx[1]);
    assert_eq!(0, idx[2]);
}

#[derive(PartialEq)]
struct Row {
    vec: Vec<String>,
}

impl Row {
    fn new() -> Row {
        Row {
            vec: Vec::with_capacity(1024),
        }
    }

    fn push(&mut self, v: &str) {
        self.vec.push(v.to_string())
    }
}

impl std::fmt::Debug for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.vec)
    }
}

fn parse(text: &[u8]) -> Vec<Row> {
    let mut r = ReaderBuilder::default().build();

    let mut data = &text[0..];
    let mut buf = [0u8; 1024];
    let mut columns = [0usize; 10];
    let (mut out_pos, mut column) = (0, 0);
    let mut vec: Vec<Row> = Vec::with_capacity(256);

    loop {
        let (state, in_size, out_size, col) = r.read_record(data, &mut buf, &mut columns[column..]);
        column += col;
        out_pos += out_size;
        data = &data[in_size..];
        match state {
            ReadResult::OutputFull => panic!("output full"),
            ReadResult::OutputEdgeFull => panic!("index full"),
            ReadResult::InputEmpty => {
                if !data.is_empty() {
                    panic!("missing input data")
                }
            }
            ReadResult::End => {
                break;
            }
            ReadResult::Record => {
                let s = std::str::from_utf8(&buf[..out_pos]).unwrap();
                if !s.is_empty() {
                    let mut row = Row::new();
                    let mut start = 0;
                    for &end in &columns[..column] {
                        row.push(&s[start..end]);
                        start = end;
                    }
                    vec.push(row);
                }
                column = 0;
                out_pos = 0;
            }
        }
    }
    vec
}
