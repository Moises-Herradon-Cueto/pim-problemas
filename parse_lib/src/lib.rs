#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub use data::get_json_string;
pub use data::read_csv;
pub use data::write_json;
pub use data::Data;
pub use files::parse_all;
pub use files::ParseOneError;

mod data;
mod files;
mod html;
mod merge;
mod parsing;
mod pdflatex;
mod preamble;
mod process_tex;

mod search;

#[derive(Serialize, Deserialize, Debug)]
pub enum Fields {
    Problem,
    Solution,
}

impl Display for Fields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Problem => f.write_str("enunciado"),
            Self::Solution => f.write_str("solución"),
        }
    }
}
