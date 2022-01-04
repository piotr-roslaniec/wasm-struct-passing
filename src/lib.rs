#![no_std]

mod utils;

use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};
use utils::set_panic_hook;

extern crate alloc;

use alloc::{collections::BTreeMap, string::String, vec::Vec};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::{
    prelude::{wasm_bindgen, JsValue},
    JsCast,
};

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct MyStruct {
    value: u32,
}

#[wasm_bindgen]
impl MyStruct {
    #[wasm_bindgen(constructor)]
    pub fn new(value: u32) -> MyStruct {
        MyStruct { value }
    }

    #[wasm_bindgen(js_name=toBytes)]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.value as u8);
        bytes
    }
}

impl MyStruct {
    pub fn from_bytes(bytes: Vec<u8>) -> MyStruct {
        let value = bytes[0] as u32;
        MyStruct { value }
    }
}

fn do_process_structs(structs: Vec<MyStruct>) -> u32 {
    structs.iter().map(|s| s.value).sum()
}

fn js_value_to_u8_vec(array_of_uint8_arrays: &[JsValue]) -> Result<Vec<Vec<u8>>, JsValue> {
    let vec_vec_u8: Vec<_> = array_of_uint8_arrays
        .iter()
        .filter_map(|u8_array| {
            u8_array
                .dyn_ref::<Uint8Array>()
                .map(|u8_array| u8_array.to_vec())
        })
        .collect();

    if vec_vec_u8.len() != array_of_uint8_arrays.len() {
        Err("Invalid Array of Uint8Arrays".into())
    } else {
        Ok(vec_vec_u8)
    }
}

#[wasm_bindgen]
pub fn process_structs(structs: Vec<JsValue>) -> u32 {
    set_panic_hook();

    let structs: Vec<MyStruct> = js_value_to_u8_vec(&structs)
        .unwrap()
        .iter()
        .cloned()
        .map(MyStruct::from_bytes)
        .collect();

    do_process_structs(structs)
}

pub type MyMap = BTreeMap<String, MyStruct>;

fn do_process_map(map: MyMap) -> u32 {
    map.into_iter()
        .map(|(s, ms)| ms.value + s.len() as u32)
        .sum()
}

#[wasm_bindgen]
pub fn process_map(map: &JsValue) -> u32 {
    set_panic_hook();

    let map: MyMap = map.into_serde().unwrap();

    do_process_map(map)
}
