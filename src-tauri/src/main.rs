#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod tweaks;
mod backup;
mod commands;
mod monitor;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_tweaks,
            commands::apply_tweak,
            commands::revert_tweak,
            commands::get_system_info,
            commands::get_backup_history,
            commands::create_backup,
            commands::restore_backup,
            commands::search_tweaks,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
