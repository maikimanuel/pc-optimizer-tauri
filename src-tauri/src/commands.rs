use crate::tweaks::{self, Tweak, TweakResult};
use crate::backup::{self, Backup};
use crate::monitor::{self, SystemInfo};

#[tauri::command]
pub fn get_tweaks() -> Result<Vec<Tweak>, String> {
    Ok(tweaks::get_all_tweaks())
}

#[tauri::command]
pub fn apply_tweak(tweak_id: String) -> Result<TweakResult, String> {
    let tweaks = tweaks::get_all_tweaks();
    let tweak = tweaks
        .iter()
        .find(|t| t.id == tweak_id)
        .ok_or("Tweak not found")?
        .clone();

    for reg_key in &tweak.registry_keys {
        tweaks::registry::set_registry_value(
            &reg_key.path,
            &reg_key.value,
            &reg_key.data.to_string(),
            &reg_key.kind,
        )?;
    }

    for service in &tweak.services {
        tweaks::services::disable_service(service)?;
    }

    Ok(TweakResult {
        success: true,
        message: format!("Successfully applied: {}", tweak.name),
        tweak_id,
    })
}

#[tauri::command]
pub fn revert_tweak(tweak_id: String) -> Result<TweakResult, String> {
    let tweaks = tweaks::get_all_tweaks();
    let tweak = tweaks
        .iter()
        .find(|t| t.id == tweak_id)
        .ok_or("Tweak not found")?
        .clone();

    for service in &tweak.services {
        tweaks::services::enable_service(service)?;
    }

    Ok(TweakResult {
        success: true,
        message: format!("Successfully reverted: {}", tweak.name),
        tweak_id,
    })
}

#[tauri::command]
pub fn get_system_info() -> Result<SystemInfo, String> {
    monitor::get_system_info()
}

#[tauri::command]
pub fn get_backup_history() -> Result<Vec<Backup>, String> {
    backup::get_backup_history()
}

#[tauri::command]
pub fn create_backup(tweaks_applied: Vec<String>) -> Result<Backup, String> {
    backup::create_backup(tweaks_applied)
}

#[tauri::command]
pub fn restore_backup(backup_id: String) -> Result<String, String> {
    Ok(format!("Restored backup: {}", backup_id))
}

#[tauri::command]
pub fn search_tweaks(query: String) -> Result<Vec<Tweak>, String> {
    let all_tweaks = tweaks::get_all_tweaks();
    let search_term = query.to_lowercase();

    let results = all_tweaks
        .into_iter()
        .filter(|t| {
            t.name.to_lowercase().contains(&search_term)
                || t.description.to_lowercase().contains(&search_term)
                || t.category.to_lowercase().contains(&search_term)
        })
        .collect();

    Ok(results)
}
