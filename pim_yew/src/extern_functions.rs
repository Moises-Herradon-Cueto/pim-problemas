use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = MathJax)]
    pub fn typeset();

    pub fn get_sheet(list: String);

    #[wasm_bindgen(js_name = uploadSheet)]
    pub async fn upload_sheet(elt_id: String, sheet_id: usize, with_solutions: bool) -> JsValue;
}
