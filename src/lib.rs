mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct MyStruct {
    value: i32,
    inner: InnerStruct,
}

#[wasm_bindgen]
impl MyStruct {
    #[wasm_bindgen(constructor)]
    pub fn new(value: i32) -> MyStruct {
        MyStruct {
            value,
            inner: InnerStruct::new(),
        }
    }
}

pub struct InnerStruct {
    value: Vec<u8>,
}

impl InnerStruct {
    pub fn new() -> InnerStruct {
        InnerStruct {
            value: vec![1, 2, 3],
        }
    }
}

#[wasm_bindgen]
pub fn greet() -> String {
    return "Hello, wasm-struct-passing!".to_string();
}
