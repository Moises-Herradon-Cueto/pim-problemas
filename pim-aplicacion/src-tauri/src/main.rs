#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;

use parse_lib::data::get_json_string;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_db_from_json])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_db_from_json(json_path: PathBuf) -> Result<Result<String, String>, ()> {
    Ok(get_db_from_json_inner(json_path))
}

fn get_db_from_json_inner(json_path: PathBuf) -> Result<String, String> {
    println!("{json_path:?}");
    let result = get_json_string(&json_path)
        .map_err(|err| format!("Error reading from {json_path:?}.\n {err}"));
    println!("Result: \n{result:#?} ");
    result
}
