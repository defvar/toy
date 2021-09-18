#![feature(test)]

extern crate serde;

extern crate test;

use test::black_box;
use test::test::Bencher;

use serde::{Deserialize, Serialize};
use toy_pack_mp::marker::marker_from_byte;
use toy_pack_mp::{pack, unpack};

#[derive(Deserialize, Serialize)]
struct TestData<'a> {
    int0: u8,
    uint1: u8,
    int1: i8,
    uint8: u8,
    int8: i16,
    uint16: u16,
    int16: i16,
    uint32: u32,
    int32: i32,
    nil: Option<u8>,
    t: bool,
    f: bool,
    ufloat: f32,
    float: f32,
    str0: &'a str,
    str1: &'a str,
    str4: &'a str,
    str8: &'a str,
    str16: &'a str,
    array0: Vec<&'a str>,
    array1: Vec<&'a str>,
}

fn get_data<'a>() -> TestData<'a> {
    TestData {
        int0: 0,
        uint1: 1,
        int1: -1,
        uint8: 255,
        int8: -255,
        uint16: 256,
        int16: -256,
        uint32: 65536,
        int32: -65536,
        nil: None,
        t: true,
        f: false,
        ufloat: 0.5,
        float: -0.5,
        str0: "",
        str1: "A",
        str4: "foobarbaz",
        str8: "Omnes viae Romam ducunt.",
        str16: "L’homme n’est qu’un roseau, le plus faible de la nature ; mais c’est un roseau pensant. Il ne faut pas que l’univers entier s’arme pour l’écraser : une vapeur, une goutte d’eau, suffit pour le tuer. Mais, quand l’univers l’écraserait, l’homme serait encore plus noble que ce qui le tue, puisqu’il sait qu’il meurt, et l’avantage que l’univers a sur lui, l’univers n’en sait rien. Toute notre dignité consiste donc en la pensée. C’est de là qu’il faut nous relever et non de l’espace et de la durée, que nous ne saurions remplir. Travaillons donc à bien penser : voilà le principe de la morale.",
        array0: vec![],
        array1: vec!["foo"],
    }
}

#[bench]
fn ser_toy(b: &mut Bencher) {
    let _ = marker_from_byte(0u8);
    let src = get_data();
    b.bytes = std::mem::size_of_val(&src) as u64;
    let vec = pack(&src);
    println!("{:?}", vec);
    b.iter(|| {
        let vec = pack(&src);
        black_box(vec)
    })
}

#[bench]
fn ser_rmp(b: &mut Bencher) {
    let src = get_data();
    b.bytes = std::mem::size_of_val(&src) as u64;
    let vec = rmp_serde::to_vec(&src).unwrap();
    println!("{:?}", vec);
    b.iter(|| {
        let vec = rmp_serde::to_vec(&src).unwrap();
        black_box(vec)
    })
}

#[bench]
fn deser_toy(b: &mut Bencher) {
    let _ = marker_from_byte(0u8);
    let src = get_data();
    let vec = pack(&src).unwrap();
    b.bytes = std::mem::size_of_val(&src) as u64;
    b.iter(|| {
        let r = unpack::<TestData>(vec.as_slice());
        black_box(r)
    })
}

#[bench]
fn deser_rmp(b: &mut Bencher) {
    let src = get_data();
    let vec = rmp_serde::to_vec(&src).unwrap();
    b.bytes = std::mem::size_of_val(&src) as u64;
    b.iter(|| {
        let r = rmp_serde::from_slice::<TestData>(&vec).unwrap();
        black_box(r)
    })
}
