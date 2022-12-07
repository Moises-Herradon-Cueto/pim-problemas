#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::let_unit_value)]

mod add_filters;
mod app;
mod column_select;
mod commands;
mod edit_entry;
mod field_display;
mod field_edit_entry;
mod field_selector;
mod files_info;
mod helper;
mod home_button;
mod main_menu;
mod raw_html;
mod result_range;
mod start;
mod update_db;
mod view_db;

use std::collections::HashMap;

use app::App;
use pim_lib::Data;

pub type DB = HashMap<usize, Data>;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    yew::start_app::<App>();
}
