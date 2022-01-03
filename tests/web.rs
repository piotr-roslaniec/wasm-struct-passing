//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::vec;

use js_sys::Uint8Array;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

use wasm_struct_passing::{process_structs, MyStruct};

#[wasm_bindgen_test]
fn test_process_structs() {
    let structs: Vec<_> = vec![MyStruct::new(1), MyStruct::new(2)]
        .iter()
        .map(|s| {
            let as_bytes = s.to_bytes();
            let as_js_value = JsValue::from_serde(&as_bytes).unwrap();
            js_sys::Uint8Array::new(&as_js_value).into()
        })
        .collect();

    let result: u32 = process_structs(structs);
    assert_eq!(result, 3);
}
