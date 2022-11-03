#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
use std::{
    fs,
    io::{self, Read},
    path::Path,
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
pub fn parse_file<T: AsRef<Path>>(path: T) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut buf = vec![];

    file.read_to_end(&mut buf)?;

    let try_read = String::from_utf8(buf.clone());

    match try_read {
        Ok(string) => return Ok(string),
        Err(err) => {
            log::info!("Not utf8:{err:?}");
        }
    }

    let mut converted_buffer = vec![0; buf.len() * 2];

    let written = convert_latin1_to_utf8(&buf, &mut converted_buffer);

    converted_buffer.truncate(written);

    Ok(String::from_utf8(converted_buffer).expect("Ni después de convertir es utf 8"))
}
