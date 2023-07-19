use std::fs;
use std::path::PathBuf;

pub fn get_file_name(dir: &str, is_folders: bool) -> Vec<PathBuf> {
    let mut file_names = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_type = entry.file_type();
                if let Ok(file_type) = file_type {
                    let file_path = entry.path();

                    if is_folders {
                        if file_type.is_dir() {
                            file_names.push(file_path);
                        }
                    } else {
                        if file_type.is_file() {
                            file_names.push(file_path);
                        }
                    }
                }
            }
        }
    }

    file_names
}
