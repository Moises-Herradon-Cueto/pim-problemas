use std::{
    collections::HashMap,
    fmt::Display,
    fs::{self, DirEntry},
    hash::BuildHasher,
    io::{self, Read},
    path::{Path, PathBuf},
};

use crate::{
    data::Data,
    merge::{string_and_data, ParseResult},
};
use encoding_rs::mem::convert_latin1_to_utf8;
use serde::{Deserialize, Serialize};

fn parse_file(path: PathBuf) -> Result<String, ParseOneError> {
    let mut file = fs::File::open(path.clone())?;
    let mut buf = vec![];

    file.read_to_end(&mut buf)?;

    let try_read = String::from_utf8(buf.clone());

    if let Ok(string) = try_read {
        return Ok(string);
    }

    let mut converted_buffer = vec![0; buf.len() * 2];

    let written = convert_latin1_to_utf8(&buf, &mut converted_buffer);

    converted_buffer.truncate(written);

    String::from_utf8(converted_buffer).map_err(|_| ParseOneError::Encoding(path))
}

pub enum ParsingError {
    IO(io::Error),
    MyError(String),
}

impl From<io::Error> for ParsingError {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<io::Error> for ParseOneError {
    fn from(value: io::Error) -> Self {
        Self::IO(value.to_string())
    }
}
#[derive(Serialize, Deserialize)]
pub enum ParseOneError {
    IO(String),
    BadFileName(String),
    NotFile(String),
    NotTex(String),
    Encoding(PathBuf),
}

impl Display for ParseOneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(err) => write!(f, "Problemas abriendo el archivo: {err}"),
            Self::BadFileName(err) => write!(
                f,
                "El archivo {err} no se llama numero.tex, p. ej. 220001.tex"
            ),
            Self::NotFile(err) => write!(f, "{err} no es un archivo"),
            Self::NotTex(err) => write!(f, "{err} no es un documento .tex"),
            Self::Encoding(err) => write!(f, "En el archivo {} no se pudo encontrar la codificación.
            \nIntenta guardarlo como utf-8, escribiendo % !TeX encoding = UTF-8 en la primera línea", err.to_string_lossy()),
        }
    }
}

fn parse_one<T: BuildHasher>(
    entry: Result<DirEntry, io::Error>,
    data: &mut HashMap<usize, Data, T>,
) -> Result<(), ParseOneError> {
    let file = entry?;
    let name = file.file_name();
    let name = name.to_string_lossy();

    if !file.file_type()?.is_file() {
        return Err(ParseOneError::NotFile(name.into_owned()));
    }
    if !name.ends_with(".tex") {
        return Err(ParseOneError::NotTex(name.into_owned()));
    }

    let id = name
        .split(".tex")
        .next()
        .ok_or_else(|| ParseOneError::BadFileName(name.clone().into_owned()))?;

    let id: Result<usize, _> = id.parse();

    let id = if let Ok(id) = id {
        id
    } else {
        return Err(ParseOneError::BadFileName(name.into_owned()));
    };
    let path = file.path();
    let in_string = parse_file(path)?;
    let out_path = format!("ejercicios-out/{name}");
    merge_file_data(id, data, &in_string, out_path)?;

    Ok(())
}

/// .
///
/// # Errors
///
/// This function will return an error if
/// the directory can't be read
///
/// # Panics
///
/// This function panics if I mess up the
/// buffer length in the call to encoding
/// ``convert_latin1_to_utf8``, inside
/// ``parse_file``
pub fn parse_all<T: BuildHasher, P: AsRef<Path>>(
    problems_dir: P,
    data: &mut HashMap<usize, Data, T>,
) -> Result<Vec<ParseOneError>, io::Error> {
    let entries = fs::read_dir(problems_dir)?;

    let mut errors = vec![];

    for file in entries {
        let res = parse_one(file, data);
        match res {
            Ok(_) => todo!(),
            Err(err) => errors.push(err),
        }
    }

    Ok(errors)
}

fn merge_file_data<T: BuildHasher, P: AsRef<Path>>(
    id: usize,
    data: &mut HashMap<usize, Data, T>,
    tex_string: &str,
    out_path: P,
) -> io::Result<()> {
    let mut placeholder = Data::new(id);
    let mut to_insert = false;
    let problem_info = data.get_mut(&id).unwrap_or_else(|| {
        println!("{id} no está en la base de datos");
        to_insert = true;
        &mut placeholder
    });
    let parse_result = string_and_data(tex_string, problem_info);
    if let ParseResult::ToChange(out_string) = parse_result {
        fs::write(out_path, out_string)?;
        if to_insert {
            data.insert(id, placeholder);
        }
    }
    Ok(())
}
