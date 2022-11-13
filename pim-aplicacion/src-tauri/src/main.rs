#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;

use parse_lib::{commands::sync_db, get_json_string};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_db_from_json, update_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_db_from_json(json_path: PathBuf) -> Result<Result<String, String>, ()> {
    Ok(get_db_from_json_inner(json_path))
}

fn get_db_from_json_inner(json_path: PathBuf) -> Result<String, String> {
    // println!("{json_path:?}");
    get_json_string(&json_path).map_err(|err| format!("Error reading from {json_path:?}.\n {err}"))
    // println!("Result: \n{result:#?} ");
}

#[tauri::command]
fn update_db(
    problems_path: PathBuf,
    db_path: PathBuf,
    output_path: PathBuf,
) -> Result<Result<String, String>, ()> {
    Ok(update_db_inner(problems_path, db_path, output_path))
}

fn update_db_inner(
    problems_path: PathBuf,
    db_path: PathBuf,
    output_path: PathBuf,
) -> Result<String, String> {
    let result = sync_db(&db_path, &problems_path, &output_path);
    serde_json::to_string(&result).map_err(|err| format!("Error converting to Json: {err}"))
}
