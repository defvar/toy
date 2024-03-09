use std::sync::Once;
use toy_core::prelude::*;
use toy_plugin_js::codec;

fn initialize() {
    static START: Once = Once::new();
    START.call_once(|| {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();
    });
}

#[test]
fn codec_object() {
    initialize();
    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    let handle_scope = &mut v8::HandleScope::new(&mut isolate);
    let context = v8::Context::new(handle_scope);
    let context = v8::Local::new(handle_scope, context);
    let scope = &mut v8::ContextScope::new(handle_scope, context);

    let map_value = map_value! {
        "a" => 1,
        "b" => true,
        "c" => "aiueo"
    };
    let frame = Frame::from_value(map_value);
    let encoded_object = codec::encode(&frame, scope);
    let decoded_value = codec::decode(encoded_object.into(), scope);

    println!("{:?}", decoded_value);
    assert_eq!(frame.into_value().unwrap(), decoded_value);
}

#[test]
fn codec_array() {
    initialize();
    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    let handle_scope = &mut v8::HandleScope::new(&mut isolate);
    let context = v8::Context::new(handle_scope);
    let context = v8::Local::new(handle_scope, context);
    let scope = &mut v8::ContextScope::new(handle_scope, context);

    let seq_value = seq_value![1u32, 2u32, 3u32];
    let frame = Frame::from_value(seq_value);
    let encoded_array = codec::encode(&frame, scope);
    let decoded_value = codec::decode(encoded_array.into(), scope);

    println!("{:?}", decoded_value);
    assert_eq!(frame.into_value().unwrap(), decoded_value);
}
