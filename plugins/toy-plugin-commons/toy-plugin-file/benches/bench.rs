#![feature(test)]

extern crate csv;
extern crate test;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use test::test::Bencher;

use toy_plugin_file::FileReaderBuilder;
use toy_text_parser::dfa::*;
use toy_text_parser::Line;

static CSV_DATA: &'static str = "./benches/bench.csv";
// header:1, data:9999
static FILE_ROW_COUNT: u32 = 10000;
static DATA_ROW_COUNT: u32 = FILE_ROW_COUNT - 1;

fn file_to_mem(fp: &str) -> Vec<u8> {
    let mut f = std::fs::File::open(fp).unwrap();
    let mut bs = vec![];
    f.read_to_end(&mut bs).unwrap();
    bs
}

#[bench]
fn read_rust_csv(b: &mut Bencher) {
    let data = file_to_mem(CSV_DATA);
    b.bytes = data.len() as u64;
    let mut count = 0;
    let mut record = csv::ByteRecord::new();
    b.iter(|| {
        let mut rdr = csv::Reader::from_reader(&*data);
        while rdr.read_byte_record(&mut record).unwrap() {
            count += 1;
        }
        assert_eq!(count, DATA_ROW_COUNT);
        count = 0;
    })
}

#[bench]
fn read_toy(b: &mut Bencher) {
    let files = vec![PathBuf::from(CSV_DATA)];
    let text = file_to_mem(CSV_DATA);
    b.bytes = text.len() as u64;
    let builder = FileReaderBuilder::default();

    let mut line = 0u32;
    let mut row = Line::new();

    b.iter(|| {
        let mut s = builder.from_file(File::open(CSV_DATA).unwrap(), files.clone());
        while s.read(&mut row).unwrap() {
            line += 1
        }
        assert_eq!(line, DATA_ROW_COUNT);
        line = 0;
    })
}

#[bench]
fn read_toy_rows_iterator(b: &mut Bencher) {
    let files = vec![PathBuf::from(CSV_DATA)];
    let text = file_to_mem(CSV_DATA);
    b.bytes = text.len() as u64;
    let builder = FileReaderBuilder::default();

    let mut line = 0u32;

    b.iter(|| {
        let mut s = builder.from_file(File::open(CSV_DATA).unwrap(), files.clone());
        for r in s.rows() {
            match r {
                Ok(_) => line += 1,
                Err(_) => panic!("error"),
            }
        }
        assert_eq!(line, DATA_ROW_COUNT);
        line = 0;
    })
}

#[bench]
fn read_toy_raw_reader(b: &mut Bencher) {
    let text = file_to_mem(CSV_DATA);
    b.bytes = text.len() as u64;
    let mut r = ByteParserBuilder::default().build();

    let mut buf = [0u8; 2048];
    let mut idx = [0usize; 30];
    let (mut idx_pos, mut count) = (0, 0);
    let mut data = &text[0..];

    b.iter(|| {
        data = &text[0..];
        loop {
            let (state, in_pos, _, col) = r.read_record(data, &mut buf, &mut idx[idx_pos..]);
            idx_pos += col;
            data = &data[in_pos..];
            match state {
                ParseResult::OutputFull => panic!("output full"),
                ParseResult::OutputEdgeFull => panic!("index full"),
                ParseResult::InputEmpty => {
                    if !data.is_empty() {
                        panic!("missing input data")
                    }
                }
                ParseResult::End => break,
                ParseResult::Record => {
                    idx_pos = 0;
                    count += 1;
                }
            }
        }
        r.reset();
        assert_eq!(count, FILE_ROW_COUNT);
        count = 0;
    })
}
