#[derive(Debug)]
pub struct Song {
    pub name: String,
    pub album: String,
    pub artist: String,
    pub artwork: String,
    pub length: u16,
}

pub mod file_scanning {

    use std::fs;
    use std::io;

    use super::Song;

    pub fn get_data(path: &str) -> Result<Vec<Song>, io::Error> {
        let mut tracks: Vec<Song> = Vec::new();
        let artists = get_folders(path)?;

        for artist in artists {
            let album_directory = format!("{}/{}", path, artist);
            let albums = get_folders(&album_directory)?;

            for album in albums {
                let album_path = format!("{}/{}", album_directory, album);
                let songs = get_files(&album_path)?;
                let artwork = get_artwork(&album_path)?;

                for song in songs {
                    let track = Song {
                        name: song,
                        album: album.to_string(),
                        artist: artist.to_string(),
                        artwork: artwork.to_string(),
                        length: 0,
                    };
                    tracks.push(track);
                }
            }
        }
        Ok(tracks)
    }

    fn get_folders(path: &str) -> Result<Vec<String>, io::Error> {
        let mut folders = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.path().is_dir() {
                folders.push(entry.file_name().to_string_lossy().to_string());
            }
        }
        Ok(folders)
    }

    fn get_files(path: &str) -> Result<Vec<String>, io::Error> {
        let track_extensions: Vec<&str> = vec![
            ".mp3", ".wav", ".m4a", ".flac", ".mp4", ".wma", ".aac", ".aiff", ".alac",
        ];

        let mut files = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.path().is_file() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if track_extensions.iter().any(|&ext| file_name.ends_with(ext)) {
                        files.push(file_name.to_string());
                    }
                }
            }
        }
        Ok(files)
    }

    fn get_artwork(path: &str) -> Result<String, io::Error> {
        let valid_filenames = vec!["cover.jpg", "cover.png", "cover.jpeg"];

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            if let Some(file_name) = entry.file_name().to_str() {
                                if valid_filenames.contains(&file_name.to_lowercase().as_str()) {
                                    let dir_path = format!("{}/{}", path.to_string(), file_name.to_string()).replace("//", "/");
                                    return Ok(dir_path)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            eprintln!("Error reading directory: {}", path);
        }

        Ok(String::new()) // return a file path of an default album cover
    }
}
