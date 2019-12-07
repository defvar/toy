use toy_pack_mp::{encoder_from_writer, EncoderOps};

#[test]
fn encode_true() {
    let mut vec: Vec<u8> = Vec::new();
    let mut encoder = encoder_from_writer(&mut vec);
    encoder.encode_bool(true).unwrap();

    assert_eq!(vec![0xc3], vec);
}

#[test]
fn encode_false() {
    let mut vec: Vec<u8> = Vec::new();
    let mut encoder = encoder_from_writer(&mut vec);
    encoder.encode_bool(false).unwrap();

    assert_eq!(vec![0xc2], vec);
}
