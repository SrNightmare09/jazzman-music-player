// DO NOT REMOVE
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![allow(unused)]
use std::collections::HashMap;
use std::fmt;

mod fetch_files;
mod database;

use rusqlite::Result;

use fetch_files::file_scanning::{self, get_data};


/*
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_files::get_folders])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
*/

fn main() {

    let music_dir: &str = "D:/Music/";

    file_scanning::get_data(music_dir);
}


// fn main() -> Result<()>{

//     // database::db::initialize();

//     Ok(())

// }
