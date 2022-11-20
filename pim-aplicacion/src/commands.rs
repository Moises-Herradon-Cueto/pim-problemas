use std::path::{Path, PathBuf};

use parse_lib::{Data, ParseOneError};
use serde::{Deserialize, Serialize};

use crate::app::invoke;

#[allow(clippy::future_not_send)]
pub async fn insert_db_info(
    problems_path: &Path,
    db_path: &Path,
    data: Data,
) -> Result<(), ParseOneError> {
    let args = InsertDbInfoArgs {
        problems_path: problems_path.to_owned(),
        db_path: db_path.to_owned(),
        data,
    };
    let args = serde_wasm_bindgen::to_value(&args).unwrap();
    let result = invoke("insert_db_info", args).await;
    serde_wasm_bindgen::from_value(result).unwrap()
}

#[derive(Serialize, Deserialize)]
struct InsertDbInfoArgs {
    #[serde(rename = "problemsPath")]
    problems_path: PathBuf,
    #[serde(rename = "dbPath")]
    db_path: PathBuf,
    data: Data,
}
