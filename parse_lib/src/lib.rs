#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

pub use data::enunciado::Enunciado;
pub use data::get_json_string;
pub use data::packages::clean as clean_packages;
pub use data::read_csv;
pub use data::write_csv;
pub use data::write_json;
pub use data::Data;
pub use data::Old as OldData;
pub use fields::FieldContents;
pub use fields::FieldContentsRef;
pub use fields::Fields;
pub use files::overwrite_file_data;
pub use files::parse_all;
pub use files::ParseOneError;
pub use files::ParseOneInfo;

pub mod commands;
mod data;
mod fields;
mod files;
mod html;
mod merge;
mod parsing;
pub mod pdflatex;
mod preamble;
mod process_tex;
pub mod table_friendly;

mod search;

pub type MsgList = Vec<(usize, files::ParseOneInfo)>;
pub type Entry = Result<(usize, files::ParseOneInfo), ParseOneError>;
