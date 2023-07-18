use std::fs;

#[tauri::command]
pub fn get_folders() -> Vec<String> {

    let directory_path: &str = "D:/Music";

    return get_dir_folders(directory_path);

}

fn get_dir_folders(directory_path: &str) -> Vec<String> {

    let mut folders: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(folder_name) = entry.file_name().into_string() {
                    let metadata = entry.metadata();
                    if let Ok(metadata) = metadata {
                        if metadata.is_dir() {
                            folders.push(folder_name);
                        }
                    }
                }
            }
        }
    } else {
        println!("Failed to read directory: {}", directory_path);
        return vec![String::from("Empty")];
    }

    return folders;


}
