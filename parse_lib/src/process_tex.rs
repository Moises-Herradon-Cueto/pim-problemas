use regex::Regex;

use crate::{data::Data, files::ParseOneError};

pub fn find_year(input: &str, data: &mut Data) -> Result<(), ParseOneError> {
    let curso = Regex::new(r"\\curso\{\s*\n(.*)\n\}")
        .map_err(|err| ParseOneError::IMessedUp(format!("I messed up the regex creation: {err}")))?
        .captures_iter(input)
        .next()
        .ok_or_else(|| ParseOneError::IMessedUp(format!("EL problema {} no tiene curso", data.id)))?
        .get(1)
        .ok_or_else(|| {
            ParseOneError::IMessedUp(format!(
                "The captured group should have an entry, parsing problem {}",
                data.id
            ))
        })?
        .as_str();
    data.curso = Some(curso.to_owned());
    Ok(())
}
