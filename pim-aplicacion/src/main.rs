#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::let_unit_value)]

mod app;
mod files_info;
mod home_button;
mod main_menu;
mod update_db;
mod view_db;

use app::App;

fn main() {
    yew::start_app::<App>();
}
