use std::io;
use std::path::Path;

use super::{get_artwork::get_artwork, get_files::get_files, get_folders::get_folders, song::Song};

pub fn get_data(path: &str) -> Result<Vec<Song>, io::Error> {
    let mut tracks: Vec<Song> = Vec::new();
    let artists = get_folders(Path::new(path))?;

    for artist in artists {
        let album_directory = Path::new(path).join(&artist);
        let albums = get_folders(&album_directory)?;

        for album in albums {
            let album_path = album_directory.join(&album);
            let songs = get_files(&album_path)?;
            let artwork = get_artwork(&album_path.to_string_lossy())?;

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
