use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

use crate::structs_js::{JsDataWrapper, JsValueWrapper};

// This is how data will come from Js
#[derive(Deserialize)]
struct JsData {
    js_num: i32,
    js_str: String,
    js_ary: Vec<i32>,
}

// This is how data will get passes back into Js
#[derive(Serialize)]
struct JsDataResult {
    js_num_squared: u64,
    js_str_rev: String,
    js_ary_sum: i64,
}

pub fn process_js_data(js_val: JsValue) -> Result<JsValue, JsValueWrapper> {
    // Serialize the JsValue object into a Rust JsData struct
    let js_data = js_val.into_serde::<JsData>()?;

    // Create a JsDataResult struct that:
    // * squares the number in js_num
    // * reverses the string in js_str
    // * sums all the values in the js_ary
    let result = JsDataResult {
        js_num_squared: u64::pow(i32::abs(js_data.js_num) as u64, 2),
        js_str_rev: js_data.js_str.chars().rev().collect::<String>(),
        js_ary_sum: js_data.js_ary.into_iter().map(|x| x as i64).sum(),
    };

    // create the object to have a 'data' property with the result above
    let data_wrapper = JsDataWrapper::new(result);

    // serialize the data_wrapper struct into a JsValue to get returned
    let js_result = JsValue::from_serde(&data_wrapper)?;
    Ok(js_result)
}
