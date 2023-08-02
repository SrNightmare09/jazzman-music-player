use rusqlite::Result;

use super::open_connection::open_connection;

pub fn clear_table(table_name: &str) -> Result<()> {
    let conn = open_connection()?;

    let sql = format!("DELETE FROM {}", table_name);

    conn.execute(&sql, [])?;

    Ok(())
}
