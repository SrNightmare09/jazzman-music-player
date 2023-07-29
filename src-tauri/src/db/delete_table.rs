use rusqlite::Result;

use super::open_connection::open_connection;

pub fn delete_table() -> Result<()> {
    let conn = open_connection()?;

    let table_name = "songs";

    let sql = format!("DROP TABLE IF EXISTS {}", table_name);

    conn.execute(&sql, [])?;

    Ok(())
}
