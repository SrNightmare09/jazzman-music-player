// DO NOT REMOVE
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io;

mod database;
mod fetch_files;

use database::db;
use fetch_files::file_scanning;

fn main() -> Result<(), io::Error> {

    let music_dir: &str = "D:/Music/";

    let data = file_scanning::get_data(music_dir);

    db::create_table();

    Ok(())
}

/*
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_files::get_folders])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
*/
