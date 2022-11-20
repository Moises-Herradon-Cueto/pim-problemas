use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    hash::BuildHasher,
    io::{self, Read},
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{
    data::Data,
    merge::{self, overwrite_document_data, ParseResult},
    MsgList,
};
use encoding_rs::mem::convert_latin1_to_utf8;
mod errors;
pub use errors::ParseOneError;
pub use errors::ParseOneInfo;

fn decode_file(path: &Path) -> Result<String, ParseOneError> {
    let mut file = fs::File::open(path).map_err(|err| ParseOneError::IO {
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

    String::from_utf8(converted_buffer).map_err(|_| ParseOneError::Encoding(path.to_path_buf()))
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
) -> Result<MsgList, ParseOneError> {
    let (path, id) = check_file(entry)?;
    let name = format!("{id}.tex");
    let in_string = decode_file(&path)?;
    let out_path =
        output_dir.join(PathBuf::from_str(&name).map_err(|_| {
            ParseOneError::IMessedUp("I'm not concatenating paths right".to_string())
        })?);

    merge_file_data(id, data, &in_string, out_path)
}

pub type OneEntry = Result<(usize, ParseOneInfo), ParseOneError>;

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
) -> Result<Vec<OneEntry>, io::Error> {
    let entries = fs::read_dir(problems_dir)?;

    let mut output = vec![];

    for file in entries {
        let result = parse_one(file, output_dir, data);
        match result {
            Err(ParseOneError::NotTex(_)) => continue,
            Ok(infos) => output.extend(infos.into_iter().map(Ok)),
            Err(error) => output.push(Err(error)),
        }
    }

    Ok(output)
}

fn merge_file_data<T: BuildHasher, P: std::fmt::Debug + Clone + AsRef<Path>>(
    id: usize,
    data: &mut HashMap<usize, Data, T>,
    tex_string: &str,
    out_path: P,
) -> Result<MsgList, ParseOneError> {
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
        ParseResult::ToChange {
            content: out_string,
            error,
            is_in_template,
        } => {
            if !is_in_template {
                return_errs.push((
                    id,
                    ParseOneInfo::NotInTemplate(out_path.as_ref().to_string_lossy().into_owned()),
                ));
            }
            return_errs.extend(error.into_iter());
            fs::write(out_path.clone(), out_string).map_err(|err| ParseOneError::IO {
                io_err: err.to_string(),
                action: format!("Error al escribir el archivo: {out_path:?}"),
            })?;
        }
        ParseResult::Template => {}
    }
    if to_insert {
        data.insert(id, placeholder);
    }
    Ok(return_errs)
}

/// .
///
/// # Errors
///
/// This function will return an error if
/// * There's a problem reading or decoding
/// the file
/// * The bit between begin document and
/// end document can't be found
/// * There's a problem writing the file
pub fn overwrite_file_data(problems_path: &Path, data: &Data) -> Result<(), ParseOneError> {
    let path = problems_path.join(format!("{}.tex", data.id));
    let file_content = decode_file(&path)?;
    let new_content = overwrite_document_data(&file_content, data)?;

    fs::write(&path, new_content).map_err(|err| ParseOneError::IO {
        io_err: err.to_string(),
        action: format!("Error al escribir el archivo: {path:?}"),
    })?;
    Ok(())
}
