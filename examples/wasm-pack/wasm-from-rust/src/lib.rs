use wasm_bindgen::prelude::*;
 
// When the 'wee_alloc' feature is enabled, use 'wee_alloc' as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
 
#[wasm_bindgen]
pub fn add_nums(num_1: i32, num_2: i32) -> i32 {
    num_1 + num_2
}
#[wasm_bindgen]
pub fn format_string(s: &str) -> String {
    format!("hello {}", s)
}
