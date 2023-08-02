use tauri::command;

use super::super::db::{insert_item, clear_table};
use super::super::fs::get_data;

#[tauri::command]
pub fn scan_music() {

    if let Err(err) = clear_table::clear_table("songs") {
        eprintln!("Error creating table: {}", err.to_string());
    }

    let data = get_data::get_data("D:/Coding/yew-test/jazzman-music-player/src/user/music/"); // TODO: add function to replace // and \\ with /

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
