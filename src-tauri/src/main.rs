// DO NOT REMOVE
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

mod fetch_files;
mod database;

use rusqlite::Result;

use fetch_files::file_scanning;


/*
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_files::get_folders])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
*/


fn main() {

    let music_dir = "D:/Music/";

    let dir_data: HashMap<String, HashMap<String, Vec<String>>> = file_scanning::get_data(music_dir);
    

}


// fn main() -> Result<()>{

//     // database::db::initialize();

//     Ok(())

// }
