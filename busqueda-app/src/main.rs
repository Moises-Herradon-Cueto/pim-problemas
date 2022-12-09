#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::let_unit_value)]

mod bindgen;
mod commands;
mod handle_db;
mod helper;
mod home_button;
mod main_menu;
mod requests;

use std::collections::HashMap;

use main_menu::MainMenu;
use pim_lib::Data;

pub type DB = HashMap<usize, Data>;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    yew::start_app::<MainMenu>();
}
