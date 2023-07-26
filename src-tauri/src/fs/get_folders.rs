use std::io;
use std::path::Path;
use std::fs;

pub fn get_folders(path: &Path) -> Result<Vec<String>, io::Error> {
    let mut folders = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.path().is_dir() {
            folders.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    Ok(folders)
}
