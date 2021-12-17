//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::vec;

use js_sys::Uint8Array;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

use wasm_struct_passing::{process_structs_1, process_structs_5, MyStruct};

// Notice that this will not work on JS side
#[wasm_bindgen_test]
fn test_process_structs_1() {
    let structs: Vec<MyStruct> = vec![MyStruct::new(1), MyStruct::new(2)];
    let result: u32 = process_structs_1(serde_wasm_bindgen::to_value(&structs).unwrap());
    assert_eq!(result, 3);
}

// This doesn't work on Rust side, but it works on JS side
// Throws:
// panicked at 'called `Result::unwrap()` on an `Err` value: JsValue("Invalid Array of Uint8Arrays")', src/lib.rs:127:10
#[wasm_bindgen_test]
fn test_process_structs_5() {
    let structs: Vec<MyStruct> = vec![MyStruct::new(1), MyStruct::new(2)];
    // let structs: Vec<Vec<u8>> = structs
    let structs: Vec<_> = structs
        .iter()
        .map(|s| {
            let as_bytes = s.to_bytes();
            JsValue::from_serde(&as_bytes).unwrap()
        })
        .collect();
    let result: u32 = process_structs_5(structs);
    assert_eq!(result, 3);
}
