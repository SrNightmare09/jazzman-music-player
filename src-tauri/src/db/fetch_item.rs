use rusqlite::Result;

use super::open_connection::open_connection;

struct Song {
    name: String,
    album: String,
    artist: String,
    artwork: String,
    length: u16,
}

pub fn fetch_item() -> Result<()> {
    let conn = open_connection()?;

    let mut sql = conn.prepare(
        "SELECT song_name, song_artist, song_album, song_artwork, song_length FROM songs"
    )?;

    let info = sql.query_map([], |row| {
        Ok(Song {
            name: row.get(0)?,
            artist: row.get(1)?,
            album: row.get(2)?,
            artwork: row.get(3)?,
            length: row.get(4)?,
        })
    })?;

    Ok(())
}
