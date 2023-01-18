use wasm_bindgen::JsCast;
use web_sys::{window, DomTokenList, Element, HtmlInputElement};
use yew::prelude::*;

pub fn _get_value_from_ref(elt: &NodeRef) -> String {
    elt.cast::<HtmlInputElement>().map_or_else(
        || String::from("Had a big problem, since this is not an input element"),
        |elt| elt.value(),
    )
}

fn class_list() -> DomTokenList {
    window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .dyn_into::<Element>()
        .unwrap()
        .class_list()
}

pub fn waiting_cursor() {
    class_list().add_1("waiting");
}

pub fn undo_waiting_cursor() {
    class_list().remove_1("waiting");
}
