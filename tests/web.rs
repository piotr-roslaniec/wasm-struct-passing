//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::vec;

use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

use wasm_struct_passing::{process_map, process_structs, MyMap, MyStruct};

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

#[wasm_bindgen_test]
fn test_process_map() {
    let map: MyMap = vec![1, 2, 3]
        .iter()
        .map(|i| {
            (
                format!("0{}", i + 1).to_string(),
                (MyStruct::new(2)),
            )
        })
        .collect();
    let map = JsValue::from_serde(&map).unwrap();

    let result: u32 = process_map(&map);
    assert_eq!(result, 12);
}
