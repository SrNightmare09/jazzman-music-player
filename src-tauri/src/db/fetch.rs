use rusqlite::params;
use tauri::command;

use super::open_connection::open_connection;

#[tauri::command]
pub fn fetch(select_qry: &str, table_qry: &str, where_qry: &str, item: &str) -> Vec<Vec<String>> {

    let conn = open_connection().unwrap();

    let mut sql: String = String::new();

    if item == "%" {
        sql = format!("SELECT {} FROM {} WHERE {} LIKE ?", select_qry, table_qry, where_qry);
    } else {
        sql = format!("SELECT {} FROM {} WHERE {} = ?", select_qry, table_qry, where_qry);
    }

    let mut statement = conn.prepare(&sql).unwrap();

    let mut items: Vec<Vec<String>> = Vec::new();

    for i in 0..count_elements(select_qry) {
        let table_row = statement.query_map(params![item], |row| {
            let item_val = row.get(i).unwrap();
            Ok(item_val)
        }).unwrap().map(|res| res.unwrap()).collect::<Vec<String>>();

        items.push(table_row);
    }

    items
}

fn count_elements(statement: &str) -> usize {
    let count = statement.chars().filter(|&char| char == ',').count();
    count + 1
}
