use rusqlite::Result;

use crate::fetch_files::Song;

use super::open_connection::open_connection;

pub fn insert_item(track: &Song) -> Result<()> {
    let conn = open_connection()?;
    let command = format!("
    INSERT INTO songs(song_name, song_artist, song_album, song_artwork, song_length)
    VALUES('{}', '{}', '{}', '{}', '{}');
    ", track.name, track.artist, track.album, track.artwork, track.length);

    let sql: &str = &command;

    conn.execute(
        sql,
        [],
    )?;

    Ok(())
}
