#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::needless_lifetimes)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{mpsc, Mutex},
    time::Duration,
};

use parse_lib::{
    commands::sync_db, get_json_string, overwrite_file_data, write_json, Data, ParseOneError,
};
use tauri::{api::dialog::FileDialogBuilder, State};
type Db = HashMap<usize, Data>;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_db_from_json,
            update_db,
            get_folder,
            insert_db_info,
            sleep
        ])
        .manage(Mutex::new(HashMap::<usize, Data>::new()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
#[allow(clippy::unnecessary_wraps)] // Err makes promise fail
#[allow(clippy::needless_pass_by_value)]
fn get_db_from_json<'r>(
    json_path: PathBuf,
    db: State<'r, Mutex<Db>>,
) -> Result<Result<String, String>, ()> {
    Ok(get_db_from_json_inner(&json_path, &db))
}

fn get_db_from_json_inner<'r>(
    json_path: &Path,
    db: &State<'r, Mutex<Db>>,
) -> Result<String, String> {
    // println!("{json_path:?}");
    let string = get_json_string(json_path)
        .map_err(|err| format!("Error reading from {json_path:?}.\n {err}"))?;
    let map: HashMap<usize, Data> = serde_json::from_str(&string)
        .map_err(|err| format!("Error deserializing in backend: {err}"))?;

    let mut lock = db.lock().map_err(|err| format!("Mutex went bad: {err}"))?;

    *lock = map;

    Ok(string)
    // println!("Result: \n{result:#?} ");
}

#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::unnecessary_wraps)] // Err makes promise fail
fn update_db(
    problems_path: PathBuf,
    db_path: PathBuf,
    output_path: PathBuf,
) -> Result<Result<String, String>, ()> {
    Ok(update_db_inner(&problems_path, &db_path, &output_path))
}

fn update_db_inner(
    problems_path: &Path,
    db_path: &Path,
    output_path: &Path,
) -> Result<String, String> {
    let result = sync_db(db_path, None, problems_path, output_path);
    serde_json::to_string_pretty(&result).map_err(|err| format!("Error converting to Json: {err}"))
}

#[tauri::command]
fn get_folder() -> Option<PathBuf> {
    let (tx, rx) = mpsc::channel();
    FileDialogBuilder::new()
        .set_directory("/home/moises/OneDrive")
        .pick_folder(move |f| {
            tx.send(f).unwrap();
        });
    rx.recv().unwrap()
}

#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::unnecessary_wraps)] // Err makes promise fail
fn insert_db_info<'r>(
    problems_path: PathBuf,
    db_path: PathBuf,
    data: Data,
    db: State<'r, Mutex<Db>>,
) -> Result<Result<(), ParseOneError>, ()> {
    Ok(insert_db_info_inner(&problems_path, &db_path, data, &db))
}

fn insert_db_info_inner<'r>(
    problems_path: &Path,
    db_path: &Path,
    data: Data,
    db: &State<'r, Mutex<Db>>,
) -> Result<(), ParseOneError> {
    let mut db = db.lock().unwrap();
    let id = data.id;
    db.insert(data.id, data);
    let data = db.get(&id).unwrap();
    overwrite_file_data(problems_path, data)?;
    write_json(db_path, &*db).map_err(|err| ParseOneError::IO {
        io_err: err.to_string(),
        action: format!("Error writing to {}", db_path.display()),
    })
}

#[tauri::command]
fn sleep(duration: Duration) {
    std::thread::sleep(duration);
}
