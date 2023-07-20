pub mod db {

    use rusqlite::{Connection, Result};

    pub fn initialize() -> Result<()> {
        let path: &str = "../db/library.db";
        let conn = open_connection(path)?;

        create_tables(&conn)?;

        Ok(())
    }

    fn open_connection(path: &str) -> Result<Connection> {
        Connection::open(path)
    }

    fn create_tables(conn: &Connection) -> Result<()> {
        conn.execute(
            "create table if not exists cat_complex (
                id integer primary key,
                name text not null unique
            )",
            [],
        )?;

        conn.execute(
            "create table if not exists cats (
                id integer primary key,
                name text not null unique,
                color_id integer not null references cat_complex(id)
            )",
            [],
        )?;

        Ok(())
    }
}