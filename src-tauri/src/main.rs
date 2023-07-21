// DO NOT REMOVE
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// use std::io;

// mod database;
// mod fetch_files;

// use database::db;
// use fetch_files::file_scanning::get_data;

// fn main() -> Result<(), io::Error> {

//     let path: &str = "D:/Music/";

//     db::create_table();

//     match get_data(path) {
//         Ok(songs) => {
//             for song in &songs {
//                 db::insert_item(song);
//             }
//         },
//         Err(err) => println!("{}", err)
//     }

//     Ok(())
// }

