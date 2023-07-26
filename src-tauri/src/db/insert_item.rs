use rusqlite::Result;

use crate::fs::song::Song;

use super::open_connection::open_connection;

pub fn insert_item(track: &Song) -> Result<()> {
    let conn = open_connection()?;
    let command = format!("
    INSERT INTO songs(song_name, song_artist, song_album, song_artwork, song_length)
    VALUES('{}', '{}', '{}', '{}', '{}');
    ", track.name.replace("'", "''"), track.artist.replace("'", "''"), track.album.replace("'", "''"), track.artwork.replace("'", "''"), track.length);

    let sql: &str = &command;

    conn.execute(
        sql,
        [],
    )?;

    Ok(())
}
