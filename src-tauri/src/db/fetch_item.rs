use rusqlite::Result;

use tauri::command;

use serde::Serialize;

use super::open_connection::open_connection;

#[derive(Serialize)]
pub struct Song {
    name: String,
    album: String,
    artist: String,
    artwork: String,
    length: u32
}

#[tauri::command]
pub fn fetch_item() -> Result<Vec<Song>, String> {
    let conn = open_connection().map_err(|err| format!("Database error: {}", err))?;

    let mut sql = conn
        .prepare("SELECT song_name, song_artist, song_album, song_artwork, song_length FROM songs")
        .map_err(|err| format!("SQLite prepare error: {}", err))?;

    let info = sql
        .query_map([], |row| {
            Ok(Song {
                name: row.get(0)?,
                artist: row.get(1)?,
                album: row.get(2)?,
                artwork: row.get(3)?,
                length: row.get(4)?,
            })
        })
        .map_err(|err| format!("SQLite query_map error: {}", err))?;

    let songs: Vec<Song> = info
        .collect::<Result<Vec<Song>, _>>()
        .map_err(|err| format!("Error collecting songs: {}", err))?;

    if songs.is_empty() {
        Err("No songs found".to_string())
    } else {
        Ok(songs)
    }
}
