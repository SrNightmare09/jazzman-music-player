#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod fs;

use db::{create_table, insert_item};
use fs::get_data;

fn main() {
    if let Err(err) = create_table::create_table() {
        eprintln!("Error creating table: {}", err.to_string());
    }

    let data = get_data::get_data("D:/Music/"); // add function to replace // and \\ with /

    match data {
        Ok(vect) => {
            for song in &vect {
                if let Err(err) = insert_item::insert_item(song) {
                    eprintln!("Error inserting item: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Error getting data: {}", err);
        }
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
