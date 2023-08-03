use tauri::command;

use super::super::db::create_table::create_table;

#[tauri::command]
pub fn initialize() {

    let songs_columns: Vec<&str> = vec!["song_name", "song_artist", "song_album", "song_artwork", "song_length", "song_id"];
    if let Err(err) = create_table("songs", songs_columns) {
        eprintln!("Error creating table: {}", err.to_string());
    } else {
        ()
    }

    let playlist_columns: Vec<&str> = vec!["playlist_name", "playlist_sngs"];
}
