#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap, path::PathBuf};

use parse_lib::{
    parse_all, {get_json_string, write_json, Data},
};

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
    db: String,
) -> Result<Result<String, String>, ()> {
    Ok(update_db_inner(problems_path, db_path, output_path, db))
}

fn update_db_inner(
    problems_path: PathBuf,
    db_path: PathBuf,
    output_path: PathBuf,
    db_json: String,
) -> Result<String, String> {
    // println!("Call update_db_inner");
    let mut db: HashMap<usize, Data> =
        serde_json::from_str(&db_json).map_err(|err| format!("Error parsing db json: {err}"))?;
    let errors = parse_all(&problems_path, &output_path, &mut db)
        .map_err(|err| format!("Error opening directory: {err}"))?;
    write_json(db_path, &db)
        .map_err(|err| format!("Found an error when writing to database: {err}"))?;
    serde_json::to_string(&errors).map_err(|err| format!("Error converting to Json: {err}"))
}
