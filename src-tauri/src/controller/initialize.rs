use tauri::command;

use super::super::db::create_table::create_table;

#[tauri::command]
pub fn initialize() {
    create_table("songs");
    create_table("playlists");
}
