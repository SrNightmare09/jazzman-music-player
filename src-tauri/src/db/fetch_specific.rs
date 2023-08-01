use rusqlite::params;
use std::collections::HashMap;
use tauri::command;

use super::open_connection::open_connection;

#[tauri::command]
pub fn fetch_specific(query: &str) -> HashMap<String, String> { // add error handling later i give up on it
    let conn = open_connection().unwrap();

    let sql = "SELECT song_album, song_artwork FROM songs WHERE song_artist = ?";

    let mut statement = conn.prepare(sql).unwrap();
    let items = statement.query_map(params![query], |row| Ok((row.get(0).unwrap(), row.get(1).unwrap()))).unwrap();

    // Create a HashMap with song_album as key and song_artwork as value
    let mut songs_map: HashMap<String, String> = HashMap::new();
    for item in items {
        if let Ok((song_album, song_artwork)) = item {
            songs_map.insert(song_album, song_artwork);
        }
    }

    songs_map
}
