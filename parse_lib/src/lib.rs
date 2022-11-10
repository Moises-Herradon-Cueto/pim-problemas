#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::borrow::Cow;
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
use Fields::{Comentarios, Difficulty, History, Packages, Problem, Solution, Source, Topics, Year};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Fields {
    Problem,
    Solution,
    Topics,
    Difficulty,
    Source,
    History,
    Comentarios,
    Year,
    Packages,
}

impl Display for Fields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Problem => f.write_str("Enunciado"),
            Solution => f.write_str("Solución"),
            Topics => f.write_str("Temas"),
            Difficulty => f.write_str("Dificultad"),
            Source => f.write_str("Fuente"),
            History => f.write_str("Historial"),
            Comentarios => f.write_str("Comentarios"),
            Year => f.write_str("Curso"),
            Packages => f.write_str("Paquetes usados"),
        }
    }
}

impl Fields {
    pub const ALL: [Self; 9] = [
        Problem,
        Solution,
        Topics,
        Difficulty,
        Source,
        History,
        Comentarios,
        Year,
        Packages,
    ];

    #[must_use]
    pub fn get(self, data: &Data) -> Cow<str> {
        match self {
            Problem => Cow::Borrowed(&data.enunciado),
            Solution => Cow::Borrowed("No están guardadas las soluciones"),
            Topics => Cow::Owned(data.temas.join(", ")),
            Difficulty => Cow::Owned(data.dificultad.to_string()),
            Source => Cow::Borrowed(&data.fuente),
            History => Cow::Owned(data.historial.join("\n")),
            Comentarios => Cow::Owned(data.comentarios.join("\n")),
            Year => Cow::Owned(data.curso.clone().unwrap_or_default()),
            Packages => Cow::Owned(data.paquetes.join("\n")),
        }
    }
}
