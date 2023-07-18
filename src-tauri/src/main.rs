// DO NOT REMOVE
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fetch_files;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_files::get_folders])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
