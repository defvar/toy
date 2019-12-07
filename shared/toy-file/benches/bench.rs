#![feature(test)]

extern crate test;

use std::io::Read;
use test::test::Bencher;

use quick_csv::Csv;

use toy_file::parse::{ReadResult, ReaderBuilder};
use toy_file::{FileReaderBuilder, Row};

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
fn read_quick_csv(b: &mut Bencher) {
    let data = file_to_mem(CSV_DATA);
    b.bytes = data.len() as u64;
    let mut count = 0;
    b.iter(|| {
        let dec = Csv::from_reader(&*data).has_header(true);
        for _row in dec.into_iter() {
            count += 1;
        }
        assert_eq!(count, DATA_ROW_COUNT);
        count = 0;
    })
}

#[bench]
fn read_toy(b: &mut Bencher) {
    let text = file_to_mem(CSV_DATA);
    b.bytes = text.len() as u64;
    let builder = FileReaderBuilder::default();

    let mut line = 0u32;
    let mut row = Row::new();

    b.iter(|| {
        let mut s = builder.from_reader(&*text);
        while s.read(&mut row).unwrap() {
            line += 1
        }
        assert_eq!(line, DATA_ROW_COUNT);
        line = 0;
    })
}

#[bench]
fn read_toy_rows_iterator(b: &mut Bencher) {
    let text = file_to_mem(CSV_DATA);
    b.bytes = text.len() as u64;
    let builder = FileReaderBuilder::default();

    let mut line = 0u32;

    b.iter(|| {
        let mut s = builder.from_reader(&*text);
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

    let mut r = ReaderBuilder::default().build();

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
                ReadResult::OutputFull => panic!("output full"),
                ReadResult::OutputEdgeFull => panic!("index full"),
                ReadResult::InputEmpty => {
                    if !data.is_empty() {
                        panic!("missing input data")
                    }
                }
                ReadResult::End => break,
                ReadResult::Record => {
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
