#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::let_unit_value)]

mod commands;
mod handle_db;
mod helper;
mod home_button;
mod main_menu;
mod requests;

use std::collections::HashMap;

use main_menu::MainMenu;
use pim_lib::Data;
use web_sys::window;

pub type DB = HashMap<usize, Data>;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    let container = window()
        .expect("Couldn't find window")
        .document()
        .expect("Couldn't find document")
        .get_element_by_id("app-container")
        .expect("Couldn't find app-container");

    yew::start_app_in_element::<MainMenu>(container);
}
