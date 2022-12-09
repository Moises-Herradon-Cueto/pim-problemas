use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn get_sheet(list: String);
}
