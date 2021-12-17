//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::vec;

use wasm_bindgen_test::*;

use wasm_struct_passing::{process_structs, MyStruct};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_process_structs_1() {
    let structs: Vec<MyStruct> = vec![MyStruct::new(1), MyStruct::new(2)];
    let result: i32 = process_structs_1(serde_wasm_bindgen::to_value(&structs).unwrap());
    assert_eq!(result, 3);
}
