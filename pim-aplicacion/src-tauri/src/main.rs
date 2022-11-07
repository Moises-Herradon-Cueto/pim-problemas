#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap, path::PathBuf};

use parse_lib::data::{read_json, Data};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_db_from_json])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

type Db = HashMap<usize, Data>;

#[tauri::command]
fn get_db_from_json(json_path: PathBuf) -> Result<Result<Db, String>, ()> {
    Ok(get_db_from_json_inner(json_path))
}

fn get_db_from_json_inner(json_path: PathBuf) -> Result<Db, String> {
    println!("{json_path:?}");
    read_json(&json_path).map_err(|err| format!("Error reading from {json_path:?}.\n {err}"))
}
