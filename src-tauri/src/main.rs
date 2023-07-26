#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fetch_files::file_scanning::get_data;

mod fetch_files;
mod db;

use db::{create_table, insert_item};

fn main() {
    create_table::create_table();
    let data = get_data("D:/Music/"); // add function to replace // and \\ with /

    match data {
        Ok(vect) => {
            for song in &vect {
                insert_item::insert_item(song);
            }
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
