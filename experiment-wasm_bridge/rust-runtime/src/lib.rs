use wasm_bindgen::prelude::*;

mod code;

#[wasm_bindgen]
pub fn get_text_js() -> String {
    code::get_text()
}