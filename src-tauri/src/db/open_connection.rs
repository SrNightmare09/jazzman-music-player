use rusqlite::{Connection, Result};

pub fn open_connection() -> Result<Connection> {
    let path: &str = "../database/library.db";
    Connection::open(path)
}
