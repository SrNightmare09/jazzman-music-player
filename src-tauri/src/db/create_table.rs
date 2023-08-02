use rusqlite::Result;

use super::open_connection::open_connection;

pub fn create_table(table_name: &str) -> Result<()> {
    let conn = open_connection()?;

    let sql = format!("CREATE TABLE {} (song_name TEXT, song_artist TEXT, song_album TEXT, song_artwork TEXT, song_length INTEGER, song_id TEXT);", table_name);

    conn.execute(&sql,[],)?;

    Ok(())
}
