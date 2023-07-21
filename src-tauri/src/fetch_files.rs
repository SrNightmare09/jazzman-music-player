#[derive(Debug)]
pub struct Song {
    name: String,
    album: String,
    artist: String,
    length: u16,
}

pub mod file_scanning {

    use std::fs;
    use std::io;
    use std::path::{Path, PathBuf};

    use super::Song;

    pub fn get_data(path: &str) -> Result<Vec<Song>, io::Error> {
        let mut tracks: Vec<Song> = Vec::new();
        let artists = get_folders(Path::new(path))?;

        for artist in artists {
            let album_directory = Path::new(path).join(&artist);
            let albums = get_folders(&album_directory)?;

            for album in albums {
                let album_path = album_directory.join(&album);
                let songs = get_files(&album_path)?;

                for song in songs {
                    let track = Song {
                        name: song,
                        album: album.to_string(),
                        artist: artist.to_string(),
                        length: 0,
                    };
                    tracks.push(track);
                }
            }
        }

        println!("{:#?}", tracks);
        Ok(tracks)
    }

    fn get_folders(path: &Path) -> Result<Vec<String>, io::Error> {
        let mut folders = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.path().is_dir() {
                folders.push(entry.file_name().to_string_lossy().to_string());
            }
        }
        Ok(folders)
    }

    fn get_files(path: &Path) -> Result<Vec<String>, io::Error> {
        let mut files = Vec::new();
        let valid_extensions: Vec<String> = vec![
            String::from(".mp3"),
            String::from(".wav"),
            String::from(".m4a")
        ];
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.path().is_file() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if valid_extensions.iter().any(|ext| file_name.ends_with(ext)) {
                        files.push(file_name.to_string());
                    }
                }
            }
        }
        Ok(files)
    }
}
