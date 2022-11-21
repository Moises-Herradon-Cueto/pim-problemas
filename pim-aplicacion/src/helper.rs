use std::time::Duration;

use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::app::invoke;

pub fn _get_value_from_ref(elt: &NodeRef) -> String {
    elt.cast::<HtmlInputElement>().map_or_else(
        || String::from("Had a big problem, since this is not an input element"),
        |elt| elt.value(),
    )
}

#[derive(Serialize, Deserialize)]
pub struct GetFolderArgs;

#[allow(clippy::future_not_send)]
pub async fn sleep(duration: Duration) {
    let args = serde_wasm_bindgen::to_value(&SleepArgs { duration })
        .expect("Couldn't make into js value🫣");
    invoke("sleep", args).await;
}

#[derive(Serialize, Deserialize)]
struct SleepArgs {
    duration: Duration,
}
