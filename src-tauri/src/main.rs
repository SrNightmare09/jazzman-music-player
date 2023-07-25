// DO NOT REMOVE
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// fn main() {
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

use database::db;
use fetch_files::file_scanning::get_data;

mod fetch_files;
mod database;

fn main() {
    database::db::create_table();
    let data = get_data("D:/Music/");

    match data {
        Ok(vect) => {
            for song in &vect {
                db::insert_item(song);
            }
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
