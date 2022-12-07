use std::{fmt::Display, path::PathBuf};

use crate::{FieldContents, Fields};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ParseOneInfo {
    NotInDb,
    NotInTemplate(String),
    NotFound(Fields),
    MissingInTex(Vec<Fields>),
    MissingInDb(Vec<Fields>),
    IMessedUp(String),
    Incompatible {
        db: FieldContents,
        tex: FieldContents,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ParseOneError {
    IO { io_err: String, action: String },
    BadFileName(String),
    NotFile(String),
    NotTex(String),
    Encoding(PathBuf),
    NotInDb(usize),
    IMessedUp(String),
    ProblemNotFound(usize),
    SolutionNotFound(usize),
}

impl std::error::Error for ParseOneError {}

impl Display for ParseOneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO { io_err, action } => write!(f, "{action}\n{io_err}"),
            Self::BadFileName(err) => write!(
                f,
                "El archivo {err} no se llama numero.tex, p. ej. 220001.tex"
            ),
            Self::NotFile(err) => write!(f, "{err} no es un archivo"),
            Self::NotTex(err) => write!(f, "{err} no es un documento .tex"),
            Self::Encoding(err) => write!(f, "En el archivo {} no se pudo encontrar la codificación.
            \nIntenta guardarlo como utf-8, escribiendo % !TeX encoding = UTF-8 en la primera línea", err.to_string_lossy()),
            Self::NotInDb(id) => write!(f, "El problema {id} no estaba en la base de datos"),
            Self::ProblemNotFound(id) => write!(f, "No se encontró el enunciado de {id} entre \\begin{{ejer}} y \\end{{ejer}}"),
            Self::SolutionNotFound(id) => write!(f, "No se encontró la solución de {id}"),
            Self::IMessedUp(msg) => f.write_str(msg)
        }
    }
}

impl std::error::Error for ParseOneInfo {}

impl Display for ParseOneInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotInDb => write!(f, "El problema no estaba en la base de datos"),
            Self::NotInTemplate(outpath) => write!(
                f,
                "El problema no estaba en la plantilla. Se ha reescrito en {outpath}"
            ),
            Self::NotFound(field) => {
                write!(f, "No se encontró el campo {field}")
            }
            Self::IMessedUp(msg) => f.write_str(msg),
            Self::MissingInTex(fields) => write!(
                f,
                "Los campos {} no está en el archivo .tex",
                fields.iter().map(|f| format!("{f}, ")).collect::<String>()
            ),
            Self::MissingInDb(fields) => write!(
                f,
                "Los campos {} no está en la base de datos",
                fields.iter().map(|f| format!("{f}, ")).collect::<String>()
            ),
            Self::Incompatible { db, tex } => write!(
                f,
                "Un campo aparece en la base de datos como \n{db}\ny en el tex como\n{tex}"
            ),
        }
    }
}
