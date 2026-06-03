use serde::{Deserialize, Serialize};
use std::fs;
use uuid::Uuid;
use chrono::Local;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backup {
    pub id: String,
    pub name: String,
    pub timestamp: String,
    pub tweaks_applied: Vec<String>,
}

pub fn create_backup(tweaks_applied: Vec<String>) -> Result<Backup, String> {
    let backup = Backup {
        id: Uuid::new_v4().to_string(),
        name: format!("Backup {}", Local::now().format("%Y-%m-%d %H:%M:%S")),
        timestamp: Local::now().to_rfc3339(),
        tweaks_applied,
    };

    let backup_dir = get_backup_dir()?;
    fs::create_dir_all(&backup_dir)
        .map_err(|e| format!("Failed to create backup directory: {}", e))?;

    let backup_file = format!("{}/{}.json", backup_dir, backup.id);
    let json_data = serde_json::to_string_pretty(&backup)
        .map_err(|e| format!("Failed to serialize backup: {}", e))?;

    fs::write(&backup_file, json_data)
        .map_err(|e| format!("Failed to write backup file: {}", e))?;

    Ok(backup)
}

pub fn get_backup_history() -> Result<Vec<Backup>, String> {
    let backup_dir = get_backup_dir()?;

    if !std::path::Path::new(&backup_dir).exists() {
        return Ok(Vec::new());
    }

    let mut backups = Vec::new();

    if let Ok(entries) = fs::read_dir(&backup_dir) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if let Ok(backup) = serde_json::from_str::<Backup>(&content) {
                            backups.push(backup);
                        }
                    }
                }
            }
        }
    }

    backups.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    Ok(backups)
}

fn get_backup_dir() -> Result<String, String> {
    let home = std::env::var("USERPROFILE").map_err(|_| "No home directory")?;
    Ok(format!("{}\\AppData\\Local\\PCOptimizer\\Backups", home))
}
