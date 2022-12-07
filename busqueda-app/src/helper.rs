use web_sys::HtmlInputElement;
use yew::prelude::*;

pub fn _get_value_from_ref(elt: &NodeRef) -> String {
    elt.cast::<HtmlInputElement>().map_or_else(
        || String::from("Had a big problem, since this is not an input element"),
        |elt| elt.value(),
    )
}
