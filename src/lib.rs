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

#[wasm_bindgen]
pub fn process_structs(structs: &JsValue) -> u32 {
    set_panic_hook();

    let structs: Vec<MyStruct> = structs.into_serde().unwrap();

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
pub fn process_structs_as_bytes(structs: Vec<JsValue>) -> u32 {
    set_panic_hook();

    let structs: Vec<MyStruct> = js_value_to_u8_vec(&structs)
        .unwrap()
        .iter()
        .cloned()
        .map(MyStruct::from_bytes)
        .collect();

    structs.iter().map(|s| s.value).sum()
}

pub type SimpleMap = BTreeMap<String, u32>;

#[wasm_bindgen]
pub fn process_map(map: &JsValue) -> u32 {
    set_panic_hook();

    let map: SimpleMap = map.into_serde().unwrap();

    map.into_iter().map(|(_, ms)| ms).sum()
}

pub type MapOfStructs = BTreeMap<String, MyStruct>;

#[wasm_bindgen]
pub fn process_map_of_struct_bytes(map: &JsValue) -> u32 {
    set_panic_hook();

    let map: BTreeMap<String, Vec<u8>> = serde_wasm_bindgen::from_value(map.clone()).unwrap();
    let map: MapOfStructs = map
        .iter()
        .map(|(s, ms)| (s.clone(), MyStruct::from_bytes(ms.to_vec())))
        .collect();

    map.into_iter().map(|(_, ms)| ms.value).sum()
}
