use regex::Regex;

use crate::data::Data;

/// .
///
/// # Panics
///
/// Panics if I mess up
pub fn find_year(input: &str, data: &mut Data) {
    let curso = Regex::new(r"\\curso\{\s*\n(.*)\n\}")
        .expect("messed up")
        .captures_iter(input)
        .next()
        .unwrap_or_else(|| panic!("El problema {} no tiene curso", data.id))
        .get(1)
        .unwrap()
        .as_str();
    data.curso = Some(curso.to_owned());
}
