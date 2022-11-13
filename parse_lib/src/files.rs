use std::{
    collections::HashMap,
    fmt::Display,
    fs::{self, DirEntry},
    hash::BuildHasher,
    io::{self, Read},
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{
    data::Data,
    merge::{self, ParseResult},
    FieldContents, Fields,
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
pub enum ParseOneInfo {
    NotInDb,
    NotInTemplate(String),
    NotFound(Fields),
    MissingInTex(Fields),
    MissingInDb(Fields),
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
    NotInDb,
    IMessedUp(String),
    ProblemNotFound,
    SolutionNotFound,
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
            Self::NotInDb => write!(f, "El problema no estaba en la base de datos"),
            Self::ProblemNotFound => write!(f, "No se encontró el enunciado entre \\begin{{ejer}} y \\end{{ejer}}"),
            Self::SolutionNotFound => write!(f, "No se encontró la solución"),
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
            Self::MissingInTex(field) => write!(f, "El campo {field} no está en el archivo .tex"),
            Self::MissingInDb(field) => write!(f, "El campo {field} no está en la base de datos"),
            Self::Incompatible { db, tex } => write!(
                f,
                "Un campo aparece en la base de datos como \n{db}\ny en el tex como\n{tex}"
            ),
        }
    }
}

fn check_file(entry: Result<DirEntry, io::Error>) -> Result<(PathBuf, usize), ParseOneError> {
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
    Ok((path, id))
}

fn parse_one<T: BuildHasher>(
    entry: Result<DirEntry, io::Error>,
    output_dir: &Path,
    data: &mut HashMap<usize, Data, T>,
) -> Result<Vec<(usize, ParseOneInfo)>, ParseOneError> {
    let (path, id) = check_file(entry)?;
    let name = id.to_string();
    let in_string = decode_file(path)?;
    let out_path =
        output_dir.join(PathBuf::from_str(&name).map_err(|_| {
            ParseOneError::IMessedUp("I'm not concatenating paths right".to_string())
        })?);

    merge_file_data(id, data, &in_string, out_path)
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
    output_dir: &Path,
    data: &mut HashMap<usize, Data, T>,
) -> Result<Vec<Result<Vec<(usize, ParseOneInfo)>, ParseOneError>>, io::Error> {
    let entries = fs::read_dir(problems_dir)?;

    let mut output = vec![];

    for file in entries {
        output.push(parse_one(file, output_dir, data));
    }

    Ok(output)
}

fn merge_file_data<T: BuildHasher, P: std::fmt::Debug + Clone + AsRef<Path>>(
    id: usize,
    data: &mut HashMap<usize, Data, T>,
    tex_string: &str,
    out_path: P,
) -> Result<Vec<(usize, ParseOneInfo)>, ParseOneError> {
    let mut placeholder = Data::new(id);
    let mut to_insert = false;
    let mut return_errs = vec![];
    let problem_info = data.get_mut(&id).unwrap_or_else(|| {
        return_errs.push((id, ParseOneInfo::NotInDb));
        to_insert = true;
        &mut placeholder
    });
    let parse_result = merge::string_and_data(tex_string, problem_info)?;
    match parse_result {
        ParseResult::ToChange(out_string, errors) => {
            return_errs.push((
                id,
                ParseOneInfo::NotInTemplate(out_path.as_ref().to_string_lossy().into_owned()),
            ));
            return_errs.extend(errors.into_iter());
            fs::write(out_path.clone(), out_string).map_err(|err| ParseOneError::IO {
                io_err: err.to_string(),
                action: format!("Error al escribir el archivo: {out_path:?}"),
            })?;
            if to_insert {
                data.insert(id, placeholder);
            }
        }
        ParseResult::Template(errors) => {
            return_errs.extend(errors.into_iter());
        }
    }
    Ok(return_errs)
}
