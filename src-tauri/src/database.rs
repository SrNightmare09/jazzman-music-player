pub mod db {

    use rusqlite::{Connection, Result};

    use crate::fetch_files::Song;

    pub fn create_table() -> Result<()> {
        let conn = open_connection()?;

        conn.execute(
            "
                CREATE TABLE songs (
                    song_name TEXT,
                    song_artist TEXT,
                    song_album TEXT,
                    song_length INTEGER
                );
            ",
            [],
        )?;

        Ok(())
    }

    pub fn insert_item(track: &Song) -> Result<()> {
        let conn = open_connection()?;
        let command = format!("
        INSERT INTO songs(song_name, song_artist, song_album, song_length)
        VALUES('{}', '{}', '{}', '{}');
        ", track.name, track.artist, track.album, track.length);

        let sql: &str = &command;

        conn.execute(
            sql,
            [],
        )?;

        Ok(())
    }

    fn open_connection() -> Result<Connection> {
        let path: &str = "../db/library.db";
        Connection::open(path)
    }
}
