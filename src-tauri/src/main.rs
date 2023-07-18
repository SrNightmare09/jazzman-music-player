// DO NOT REMOVE
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fetch_files;

use std::collections::HashMap;

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
    let artists = fetch_files::scan_directory(&music_dir);

    let mut albums: HashMap<&String, Vec<String>> = HashMap::new();

    for artist in &artists {

        let directory = format!("{music_dir}/{artist}");
        albums.insert(artist, fetch_files::scan_directory(&directory));

    }

    println!("{:#?}", albums);

}
