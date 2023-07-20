
#[derive(Debug)]
pub struct Track {
    name: String,
}

#[derive(Debug)]
pub struct Album {
    name: String,
    tracks: Vec<Track>,
}

#[derive(Debug)]
pub struct Artist {
    name: String,
    albums: Vec<Album>,
}

impl Album {
    fn new(name: String) -> Self {
        Self {
            name,
            tracks: Vec::new(),
        }
    }

    fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }
}

impl Artist {
    fn new(name: String) -> Self {
        Self {
            name,
            albums: Vec::new(),
        }
    }

    fn add_album(&mut self, album: Album) {
        self.albums.push(album);
    }
}

pub mod file_scanning {

    use std::{collections::HashMap, fs};

    use super::Artist;
    use super::Track;
    use super::Album;

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

    pub fn get_data(path: &str) -> HashMap<String, Artist> {
        let artists = get_folders(&path);

        let mut artists_data: HashMap<String, Artist> = HashMap::new();

        for artist in &artists {
            // we get all the albums of an artist
            let album_directory = format!("{}/{}", &path, artist);
            let albums = get_folders(&album_directory);

            let mut artist_data = Artist::new(artist.to_string());
            // println!("{:?}", artist_data);

            for album in &albums {
                let this_alb_path = format!("{}/{}", &album_directory, album);
                let tracks = get_files(&this_alb_path);

                let mut album_data = Album::new(album.to_string());

                for track in &tracks {
                    album_data.add_track(Track {
                        name: track.to_string(),
                    });
                }

                artist_data.add_album(album_data);
            }

            artists_data.insert(artist.to_string(), artist_data);
        }

        println!("{:#?}",& artists_data);
        artists_data
    }
}
