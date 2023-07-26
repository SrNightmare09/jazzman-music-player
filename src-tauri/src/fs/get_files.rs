use std::io;
use std::path::Path;
use std::fs;

pub fn get_files(path: &Path) -> Result<Vec<String>, io::Error> {
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