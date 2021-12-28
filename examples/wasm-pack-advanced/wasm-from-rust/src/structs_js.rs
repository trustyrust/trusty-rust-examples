use serde::Serialize;
use wasm_bindgen::JsValue;

// This is a simple wrapper for JsValue
// used below to create a custom impl for serde_json::Error
pub struct JsValueWrapper {
    pub js_value: JsValue,
}
// This impl<From> is how we use the ? early return operator
impl From<serde_json::Error> for JsValueWrapper {
    fn from(err: serde_json::Error) -> Self {
        Self {
            js_value: JsValue::from_serde(&JsErrWrapper {
                err: JsErr {
                    message: err.to_string(),
                },
            })
            .unwrap(),
        }
    }
}

// Simple struct to later serialize into to make a Js Error Object
// Follows Node Like naming convention in event of error
/*
    { err: { message: 'err msg' } }
*/
#[derive(Serialize)]
pub struct JsErrWrapper {
    pub err: JsErr,
}
#[derive(Serialize)]
pub struct JsErr {
    pub message: String,
}

// Simple struct to later serialize into to make a Js Error Object
/*
    { data: { ... } }
*/
#[derive(Serialize)]
pub struct JsDataWrapper<T> {
    data: T,
}
impl<T> JsDataWrapper<T> {
    pub fn new(t: T) -> Self {
        Self { data: t }
    }
}
