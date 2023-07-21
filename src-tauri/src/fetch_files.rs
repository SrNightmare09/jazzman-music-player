#[derive(Debug)]
pub struct Song {
    name: String,
    album: String,
    artist: String,
    length: u16,
}

pub mod file_scanning {

    use std::{collections::HashMap, fs};

    use crate::fetch_files::Song;

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
        let valid_extensions: Vec<String> = vec![
            String::from(".mp3"),
            String::from(".wav"),
            String::from(".m4a")
        ];

        if let Ok(entries) = fs::read_dir(directory_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_name) = entry.file_name().into_string() {

                        if valid_extensions.iter().any(|ext| file_name.ends_with(ext)) {
                            files.push(file_name);
                        }

                    }
                }
            }
        } else {
            return vec![String::from("empty")];
        }

        files
    }

    pub fn get_data(path: &str) -> HashMap<String, HashMap<String, Vec<String>>> {
        let artists = get_folders(&path);

        let mut tracks: Vec<Song> = Vec::new();

        let mut _albums: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

        for artist in &artists {
            // we get all the albums of an artist
            let album_directory = format!("{}/{}", &path, artist);
            let albums = get_folders(&album_directory);

            let mut album_tracks: HashMap<String, Vec<String>> = HashMap::new();

            for album in &albums {
                let this_alb_path = format!("{}/{}", &album_directory, album);
                let songs = get_files(&this_alb_path);

                for song in &songs {

                    let track: Song = Song {
                        name: song.to_string(),
                        album: album.to_string(),
                        artist: artist.to_string(),
                        length: 0
                    };

                    tracks.push(track);

                }

            }

            _albums.insert(artist.to_string(), album_tracks);
        }

        println!("{:#?}", tracks);

        _albums
    }
}
