// DO NOT REMOVE
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

#[tauri::command]
fn test_func(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn fetch_files() -> String {
    println!("fetching files...");

    let directory_path: &str = "directory";
    list_files_in_directory(directory_path);

    return format!("this is a test");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test_func, fetch_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn list_files_in_directory(directory_path: &str) {
    if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_name) = entry.file_name().into_string() {
                    println!("File: {}", file_name);
                }
            }
        }
    } else {
        println!("Failed to read directory: {}", directory_path);
    }
}
