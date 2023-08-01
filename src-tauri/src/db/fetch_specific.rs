use rusqlite::{params, Result};
use std::collections::HashMap;

use super::open_connection::open_connection;

pub fn fetch_specific(query: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let conn = open_connection().map_err(|err| format!("Database error: {}", err))?;

    let sql = "SELECT song_album, song_artwork FROM songs WHERE song_artist = ?";

    let mut statement = conn.prepare(sql)?;
    let items = statement.query_map(params![query], |row| Ok((row.get(0)?, row.get(1)?)))?;

    // Create a HashMap with song_album as key and song_artwork as value
    let mut songs_map: HashMap<String, String> = HashMap::new();
    for item in items {
        if let Ok((song_album, song_artwork)) = item {
            songs_map.insert(song_album, song_artwork);
        }
    }

    // Print out the contents of the HashMap
    println!("{:#?}", songs_map);

    Ok(songs_map)
}
