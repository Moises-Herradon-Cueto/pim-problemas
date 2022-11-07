#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap, path::Path};

use parse_lib::data::{read_json, Data};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_db_from_json])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn update_db(db_path: &Path, problems_path: &Path) {}

#[tauri::command]
fn get_db_from_json(json_path: &Path) -> Result<HashMap<usize, Data>, String> {
    read_json(json_path).map_err(|err| err.to_string())
}
