use rusqlite::{params, Connection, Result};

use super::open_connection::open_connection;

pub fn delete_all_data_in_table() -> Result<()> {
    // Open a connection to the database
    ;
    let conn = open_connection()?;

    // Specify the name of the table from which you want to delete all data
    let table_name = "songs";

    // SQL query to delete all data from the table
    let sql = format!("DELETE FROM {}", table_name);

    // Execute the query
    conn.execute(&sql, params![])?;

    // All data in the table is now deleted
    println!("All data in table {} has been deleted.", table_name);

    Ok(())
}
