use rusqlite::Result;

use super::open_connection::open_connection;

pub fn create_table() -> Result<()> {
    let conn = open_connection()?;

    conn.execute(
        "
                CREATE TABLE songs (
                    song_name TEXT,
                    song_artist TEXT,
                    song_album TEXT,
                    song_artwork TEXT,
                    song_length INTEGER
                );
            ",
        [],
    )?;

    Ok(())
}
