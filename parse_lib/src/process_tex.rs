use regex::Regex;

use crate::data::{self, Data};

/// .
///
/// # Panics
///
/// Panics if I fuck up
pub fn find_year(input: &str, data: &mut Data) {
    let curso = Regex::new(r"\\curso\{\s*\n(.*)\n\}")
        .expect("fucked up")
        .captures_iter(input)
        .next()
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    data.curso = Some(curso.to_owned());
}
