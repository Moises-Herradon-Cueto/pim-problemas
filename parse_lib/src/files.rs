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
    merge::{self, ParseResult},
    Fields,
};
use encoding_rs::mem::convert_latin1_to_utf8;
use serde::{Deserialize, Serialize};

fn decode_file(path: PathBuf) -> Result<String, ParseOneError> {
    let mut file = fs::File::open(path.clone()).map_err(|err| ParseOneError::IO {
        io_err: err.to_string(),
        action: format!("Error al abrir el archivo: {path:?}"),
    })?;
    let mut buf = vec![];

    file.read_to_end(&mut buf)
        .map_err(|err| ParseOneError::IO {
            io_err: err.to_string(),
            action: format!("Error al leer el archivo: {path:?}"),
        })?;

    let try_read = String::from_utf8(buf.clone());

    if let Ok(string) = try_read {
        return Ok(string);
    }

    let mut converted_buffer = vec![0; buf.len() * 2];

    let written = convert_latin1_to_utf8(&buf, &mut converted_buffer);

    converted_buffer.truncate(written);

    String::from_utf8(converted_buffer).map_err(|_| ParseOneError::Encoding(path))
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ParseOneError {
    IO { io_err: String, action: String },
    BadFileName(String),
    NotFile(String),
    NotTex(String),
    Encoding(PathBuf),
    NotInDb(usize),
    NotInTemplate(usize, String),
    NotFound(usize, Fields),
    IMessedUp(String),
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
            Self::NotInTemplate(id, outpath) => write!(f, "El problema {id} no estaba en la plantilla. Se ha reescrito en {outpath}"),
            Self::NotFound(id, field) => write!(f, "No se encontró el campo {field} en el problema {id}"),
            Self::IMessedUp(msg) => f.write_str(msg)
        }
    }
}

fn parse_one<T: BuildHasher>(
    entry: Result<DirEntry, io::Error>,
    data: &mut HashMap<usize, Data, T>,
) -> Result<(), ParseOneError> {
    let file = entry.map_err(|err| ParseOneError::IO {
        io_err: err.to_string(),
        action: "Error en la entrada del directorio?".to_string(),
    })?;
    let name = file.file_name();
    let name = name.to_string_lossy();

    if !file
        .file_type()
        .map_err(|err| ParseOneError::IO {
            io_err: err.to_string(),
            action: "Error al buscar el tipo de archivo".to_string(),
        })?
        .is_file()
    {
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
    let in_string = decode_file(path)?;
    let out_path = format!("/home/moises/ejercicios-out/{name}");
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
            Ok(_) => {}
            Err(err) => errors.push(err),
        }
    }

    Ok(errors)
}

fn merge_file_data<T: BuildHasher, P: std::fmt::Debug + Clone + AsRef<Path>>(
    id: usize,
    data: &mut HashMap<usize, Data, T>,
    tex_string: &str,
    out_path: P,
) -> Result<Vec<ParseOneError>, ParseOneError> {
    let mut placeholder = Data::new(id);
    let mut to_insert = false;
    let mut return_errs = vec![];
    let problem_info = data.get_mut(&id).unwrap_or_else(|| {
        return_errs.push(ParseOneError::NotInDb(id));
        to_insert = true;
        &mut placeholder
    });
    let parse_result = merge::string_and_data(tex_string, problem_info);
    match parse_result {
        Err(err) => return_errs.push(err),
        Ok(ParseResult::ToChange(out_string)) => {
            return_errs.push(ParseOneError::NotInTemplate(
                id,
                out_path.as_ref().to_string_lossy().into_owned(),
            ));
            fs::write(out_path.clone(), out_string).map_err(|err| ParseOneError::IO {
                io_err: err.to_string(),
                action: format!("Error al escribir el archivo: {out_path:?}"),
            })?;
            if to_insert {
                data.insert(id, placeholder);
            }
        }
        Ok(ParseResult::Template) => {}
    }
    Ok(return_errs)
}
