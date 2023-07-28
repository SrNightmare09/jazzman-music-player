use rusqlite::Result;

use tauri::command;

use super::open_connection::open_connection;

#[tauri::command]
pub fn fetch_item(item: &str) -> Result<Vec<String>, String> {
    let conn = open_connection().map_err(|err| format!("Database error: {}", err))?;

    let mut sql = conn
        .prepare(&format!("SELECT {} FROM songs", item))
        .map_err(|err| format!("SQLite prepare error: {}", err))?;

    let info = sql
        .query_map([], |row| {
            Ok(row.get(0)?)
        })
        .map_err(|err| format!("SQLite query_map error: {}", err))?
        .collect::<Result<Vec<String>, _>>() // Collect into Vec<String>
        .map_err(|err| format!("SQLite collect error: {}", err))?;

    Ok(info)
}
