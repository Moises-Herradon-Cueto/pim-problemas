use std::{
    collections::HashMap,
    fmt::Debug,
    fs,
    hash::BuildHasher,
    io::{self, Read},
    path::Path,
};

use crate::{
    data::Data,
    merge::{string_and_data, ParseResult},
};
use encoding_rs::mem::convert_latin1_to_utf8;

/// .
///
/// # Panics
///
/// Panics if I mess up the implementation
///
/// # Errors
///
/// This function will return an error if
/// * The file can't be opened
/// * There's an error reading the file
pub fn parse_file<T: Clone + Debug + AsRef<Path>>(path: T) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut buf = vec![];

    file.read_to_end(&mut buf)?;

    let try_read = String::from_utf8(buf.clone());

    if let Ok(string) = try_read {
        return Ok(string);
    }

    let mut converted_buffer = vec![0; buf.len() * 2];

    let written = convert_latin1_to_utf8(&buf, &mut converted_buffer);

    converted_buffer.truncate(written);

    Ok(String::from_utf8(converted_buffer).expect("Ni después de convertir es utf 8"))
}

/// .
///
/// # Errors
///
/// This function will return an error if there's an io problem
///
/// # Panics
///
/// If I mess up
pub fn parse_all<T: BuildHasher>(data: &mut HashMap<usize, Data, T>) -> io::Result<()> {
    let entries = fs::read_dir("ejercicios-in")?;

    for file in entries {
        let file = file?;
        let name = file.file_name();
        let name = name.to_string_lossy();

        if file.file_type()?.is_file() && name.ends_with(".tex") {
            let id: Result<usize, _> = name.split(".tex").next().unwrap().parse();

            let id = if let Ok(id) = id {
                id
            } else {
                println!("Nombre de archivo {name}");
                continue;
            };
            let path = file.path();
            let in_string = parse_file(path).expect("Had problems parsing file");
            let out_path = format!("ejercicios-out/{name}");
            merge_file_data(id, data, &in_string, out_path)?;
        }
    }

    Ok(())
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
