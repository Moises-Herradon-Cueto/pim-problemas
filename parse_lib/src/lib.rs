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
    pub fn get_string(self, data: &Data) -> Cow<str> {
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

    #[must_use]
    pub fn get(self, data: &Data) -> FieldContentsRef {
        match self {
            Problem => FieldContentsRef::Str(&data.enunciado),
            Solution => FieldContentsRef::Optional(&None),
            Difficulty => FieldContentsRef::Difficulty(&data.dificultad),
            Topics => FieldContentsRef::VecStr(&data.temas),
            Source => FieldContentsRef::Str(&data.fuente),
            History => FieldContentsRef::VecStr(&data.historial),
            Comentarios => FieldContentsRef::VecStr(&data.comentarios),
            Year => FieldContentsRef::Optional(&data.curso),
            Packages => FieldContentsRef::VecStr(&data.paquetes),
        }
    }
}

pub enum FieldContents {
    Id(usize),
    VecStr(Vec<String>),
    Difficulty(u8),
    Str(String),
    Optional(Option<String>),
}

#[derive(PartialEq, Eq)]
pub enum FieldContentsRef<'a> {
    Id(&'a usize),
    VecStr(&'a [String]),
    Difficulty(&'a u8),
    Str(&'a str),
    Optional(&'a Option<String>),
}

impl<'a> FieldContentsRef<'a> {
    #[must_use]
    pub fn to_owned(&self) -> FieldContents {
        match self {
            Self::Id(x) => FieldContents::Id(**x),
            Self::VecStr(x) => FieldContents::VecStr((*x).to_vec()),
            Self::Difficulty(x) => FieldContents::Difficulty(**x),
            Self::Str(x) => FieldContents::Str((*x).to_owned()),
            Self::Optional(x) => FieldContents::Optional(x.as_ref().cloned()),
        }
    }
}
