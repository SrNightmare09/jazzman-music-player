use std::fs;

#[tauri::command]
pub fn get_files() -> String {

    let directory_path: &str = "D:/Music";

    get_dir_folders(directory_path);

    return format!("this is a test");
}

fn get_dir_folders(directory_path: &str) {

    let mut folders: Vec<fs::Metadata> = Vec::new();

    if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(_file_name) = entry.file_name().into_string() {
                    let metadata = entry.metadata();
                    if let Ok(metadata) = metadata {
                        if metadata.is_dir() {
                            folders.push(metadata);
                        }
                    }
                }
            }
        }
    } else {
        println!("Failed to read directory: {}", directory_path);
    }
}
