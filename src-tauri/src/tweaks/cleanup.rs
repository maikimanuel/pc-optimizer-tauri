use std::fs;
use std::path::Path;

pub fn clean_temp_files() -> Result<u64, String> {
    let mut total_freed = 0u64;

    if let Ok(size) = clean_directory("C:\\Windows\\Temp") {
        total_freed += size;
    }

    if let Ok(size) = clean_directory(&format!(
        "C:\\Users\\{}\\AppData\\Local\\Temp",
        std::env::var("USERNAME").unwrap_or_default()
    )) {
        total_freed += size;
    }

    Ok(total_freed)
}

fn clean_directory(path: &str) -> Result<u64, String> {
    let mut total_size = 0u64;

    if !Path::new(path).exists() {
        return Ok(0);
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                total_size += metadata.len();
                let path = entry.path();
                
                if metadata.is_dir() {
                    let _ = fs::remove_dir_all(&path);
                } else {
                    let _ = fs::remove_file(&path);
                }
            }
        }
    }

    Ok(total_size)
}
