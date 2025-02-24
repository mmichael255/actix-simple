use serde::Serialize;
use serde_wasm_bindgen::Error as SerdeWasmBindgenError;

#[derive(Debug, Serialize)]
pub enum MyError {
    SomeError(String),
}

impl From<String> for MyError {
    fn from(s: String) -> Self {
        Self::SomeError(s)
    }
}

impl From<wasm_bindgen::JsValue> for MyError {
    fn from(js_value: wasm_bindgen::JsValue) -> Self {
        MyError::SomeError(js_value.as_string().unwrap())
    }
}

impl From<SerdeWasmBindgenError> for MyError {
    fn from(e: SerdeWasmBindgenError) -> Self {
        Self::SomeError(e.to_string())
    }
}
