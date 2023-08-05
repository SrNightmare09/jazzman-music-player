#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod controller;
mod db;
mod fs;

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            controller::scan_music::scan_music,
            db::fetch_item::fetch_item,
            db::fetch_specific_artwork::fetch_specific_artwork,
            db::fetch_one_to_one::fetch_one_to_one,
            controller::initialize::initialize
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
