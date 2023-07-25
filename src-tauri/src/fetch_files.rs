#[derive(Debug)]
pub struct Song {
    pub name: String,
    pub album: String,
    pub artist: String,
    pub artwork: String,
    pub length: u16,
}

pub mod file_scanning {

    use std::fmt::format;
    use std::fs;
    use std::io;
    use std::path::Path;
    use std::path::PathBuf;

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
        let track_extensions: Vec<String> = vec![
            String::from(".mp3"),
            String::from(".wav"),
            String::from(".m4a"),
            String::from(".flac"),
            String::from(".mp4"),
            String::from(".wma"),
            String::from(".aac"),
            String::from(".aiff"),
            String::from(".alac"),
        ];

        let mut files = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.path().is_file() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if track_extensions.iter().any(|ext| file_name.ends_with(ext)) {
                        files.push(file_name.to_string());
                    }
                }
            }
        }
        Ok(files)
    }

    fn get_artwork(path: &Path) -> Result<String, io::Error> {
        let artwork_extensions: Vec<String> = vec![
            String::from(".jpg"),
            String::from(".jpeg"),
            String::from(".png"),
            String::from(".gif"),
        ];

        let mut artwork: String = String::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.path().is_file() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if artwork_extensions
                        .iter()
                        .any(|ext| file_name.ends_with(&format!("Cover{}", ext)))
                    {
                        let art_path: PathBuf = path.join(file_name);
                        artwork = art_path.into_os_string().into_string().unwrap().replace("\\", "/");
                    }
                }
                else {
                    ()
                }
            }
        }
        Ok(artwork)
    }
}
