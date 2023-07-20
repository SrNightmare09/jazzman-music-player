pub mod file_scanning {

    use std::{fs, collections::HashMap};

    fn get_folders(directory_path: &str) -> Vec<String> {
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

        folders
    }

    fn get_files(directory_path: &str) -> Vec<String> {
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

                        if file_name.ends_with(".mp3") || file_name.ends_with(".wav") {
                            files.push(file_name);
                        }
                    }
                }
            }
        } else {
            return vec![String::from("Empty")];
        }

        files
    }

    pub fn get_data(path: &str) {
        let artists = get_folders(&path);

        let mut _albums: HashMap<&String, HashMap<String, Vec<String>>> = HashMap::new();

        for artist in &artists {
            // we get all the albums of an artist
            let album_directory = format!("{path}/{artist}");
            let albums = get_folders(&album_directory);

            let mut album_tracks: HashMap<String, Vec<String>> = HashMap::new();

            for album in &albums {
                let this_alb_path = format!("{}/{album}", &album_directory);
                album_tracks.insert(album.to_string(), get_files(&this_alb_path));
            }

            _albums.insert(artist, album_tracks);
        }

        println!("{:#?}", _albums);
    }
}
