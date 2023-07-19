// #[tauri::command]
/*
pub fn get_folders() -> Vec<String> {

    let directory_path: &str = "D:/Music/Eminem";

    return scan_directory(directory_path);

}
*/

/*
enum Song {
    SongArtist(String),
    AlbumArtist(String),
    Album(String),
    Year(u32),
    LengthSeconds(u32)
}

enum Artist {
    Name(String),
    NoOfAlbums(u32),
    AlbumNames(Vec<String>)
}
*/

pub mod file_scanning {

    use std::fs;

    pub fn get_folders(directory_path: &str) -> Vec<String> {
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

    pub fn get_files(directory_path: &str) -> Vec<String> {

        let mut files: Vec<String> = Vec::new();
        // let valid_extensions: Vec<String> = vec![String::from("mp3"), String::from("wav"), String::from("mpeg")];

        if let Ok(entries) = fs::read_dir(directory_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_name) = entry.file_name().into_string() {

                        // for valid_ext in valid_extensions {
                        //     if (file_name.ends_with(valid_ext)) {
                        //         files.push(file_name);
                        //     }
                        // }

                        if (file_name.ends_with(".mp3") || file_name.ends_with(".wav")) {
                            files.push(file_name);
                        }

                    }
                }
            }
        } else {
            return vec![String::from("empty")];
        }

        return files;
    }

}
