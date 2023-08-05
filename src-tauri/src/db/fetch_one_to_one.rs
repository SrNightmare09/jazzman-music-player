use rusqlite::params;
use std::collections::HashMap;
use tauri::command;

use super::open_connection::open_connection;

#[tauri::command]
pub fn fetch_one_to_one(select_qry: &str, where_qry: &str, query: &str) -> Vec<String> { // add error handling later i give up on it
    let conn = open_connection().unwrap();

    let sql = format!("SELECT {} FROM songs WHERE {} = ?", select_qry, where_qry);

    let mut statement = conn.prepare(&sql).unwrap();
    let items = statement.query_map(params![query], |row| Ok(row.get(0).unwrap())).unwrap();

    // Create a HashMap with song_album as key and song_artwork as value
    let mut songs_vec: Vec<String> = Vec::new();
    for item in items {
        if let Ok(song_name) = item {
            songs_vec.push(song_name);
        }
    }

    songs_vec
}
