// DO NOT REMOVE
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

mod fetch_files;

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
    let artists = file_scanning::get_folders(&music_dir);

    let mut _albums: HashMap<&String, HashMap<String, Vec<String>>> = HashMap::new();

    for artist in &artists {

        // we get all the albums of an
        let album_directory = format!("{music_dir}/{artist}");
        let albums = file_scanning::get_folders(&album_directory);

        let mut album_tracks: HashMap<String, Vec<String>> = HashMap::new();

        for album in &albums {

            let this_alb_path = format!("{}/{album}", &album_directory);
            album_tracks.insert(album.to_string(), file_scanning::get_files(&this_alb_path));

        }

        _albums.insert(artist, album_tracks);

    }

    println!("{:#?}", _albums);

}
