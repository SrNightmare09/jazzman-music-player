use std::io;
use std::path::Path;

use rand::distributions::Alphanumeric;
use rand::Rng;

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
                    id: generate_id()
                };
                tracks.push(track);
            }
        }
    }
    Ok(tracks)
}

fn generate_id() -> String {
    let rng = rand::thread_rng();
    let random_string: String = rng.sample_iter(&Alphanumeric).take(10).map(char::from).collect();

    random_string
}
