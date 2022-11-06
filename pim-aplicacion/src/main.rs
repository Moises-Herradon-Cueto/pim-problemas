#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::let_unit_value)]

mod app;
mod main_menu;
mod update;

use app::App;

fn main() {
    yew::start_app::<App>();
}
