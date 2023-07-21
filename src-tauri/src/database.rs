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

    fn open_connection() -> Result<Connection> {
        let path: &str = "../db/library.db";
        Connection::open(path)
    }
}
