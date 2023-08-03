use rusqlite::Result;

use super::open_connection::open_connection;

pub fn create_table(table_name: &str, columns: Vec<&str>) -> Result<()> {
    let conn = open_connection()?;

    let fields: String = {
        let mut statement = String::new();
        for column in &columns {
            let field = format!("{} {}, ", column, get_data_type(column));
            statement.push_str(&field);
        }
        statement.pop();
        statement.pop(); // remove comma and space from end
        statement
    };

    let sql = format!("CREATE TABLE {} ({});", table_name, &fields);

    conn.execute(&sql,[],)?;

    Ok(())
}

fn get_data_type(column: &str) -> String {
    let text = vec!["song_name", "song_artist", "song_album", "song_artwork"];
    let integer = vec!["song_length"];
    let var_char = vec!["song_id"];

    for item in &text {
        if &column == item {
            return String::from("TEXT");
        }
    }

    for item in &integer {
        if &column == item {
            return String::from("INTEGER");
        }
    }

    for item in &var_char {
        if &column == item {
            return String::from("VARCHAR(10)");
        }
    }
    return String::from("UNKNOWN");
}
