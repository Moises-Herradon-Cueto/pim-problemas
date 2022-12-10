use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = MathJax)]
    pub fn typeset();

    pub fn get_sheet(list: String);

}
