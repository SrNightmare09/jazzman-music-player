use std::fs;
use std::io;

pub fn get_artwork(path: &str) -> Result<String, io::Error> {
    let valid_filenames = vec!["cover.jpg", "cover.png", "cover.jpeg"];

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(file_name) = entry.file_name().to_str() {
                            if valid_filenames.contains(&file_name.to_lowercase().as_str()) {
                                let dir_path =
                                    format!("{}/{}", path.to_string(), file_name.to_string())
                                        .replace("//", "/")
                                        .replace("\\", "/");
                                return Ok(dir_path);
                            }
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("Error reading directory: {}", path);
    }

    Ok(String::from("assets/default_cover.png")) // return a file path of an default album cover
}
