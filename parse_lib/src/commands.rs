use std::{collections::HashMap, path::Path};

use crate::{get_json_string, parse_all, write_json, Data, Entry};

/// # Errors
///
/// This function will return an error if
/// there's an IO or a serialization error
pub fn sync_db(
    database_dir: &Path,
    problems_dir: &Path,
    output_dir: &Path,
) -> Result<Vec<Entry>, String> {
    let data = get_json_string(database_dir).map_err(|err| match err {
        crate::data::Error::IO(x) => format!("IO Error: {x}"),
        crate::data::Error::Serde(x) => format!("Error deserializing: {x}"),
    })?;
    let mut data: HashMap<usize, Data> =
        serde_json::from_str(&data).map_err(|err| format!("Error deserializing: {err}"))?;
    for value in data.values_mut() {
        value.trim();
    }
    let result = parse_all(problems_dir, output_dir, &mut data)
        .map_err(|err| format!("Error opening tex files: {err}"))?;
    write_json(database_dir, &data)
        .map_err(|err| format!("Failed to serialize and write: {err}"))?;
    Ok(result)
}
