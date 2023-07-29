use tauri::command;

use super::super::db::{insert_item, create_table, delete_table};
use super::super::fs::get_data;

#[tauri::command]
pub fn scan_music() {

    if let Err(err) = delete_table::delete_table() {
        eprintln!("Error creating table: {}", err.to_string());
    }

    if let Err(err) = create_table::create_table() {
        eprintln!("Error creating table: {}", err.to_string());
    }

    let data = get_data::get_data("D:/Coding/yew-test/jazzman-music-player/src/user/music/"); // add function to replace // and \\ with /

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
}
