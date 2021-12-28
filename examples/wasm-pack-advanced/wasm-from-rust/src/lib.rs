use process_data::process_js_data;
use wasm_bindgen::prelude::*;

mod process_data;
mod structs_js;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn data_processer(js_val: JsValue) -> JsValue {
    match process_js_data(js_val) {
        Ok(d) => d,
        Err(e) => e.js_value,
    }
}
