pub mod registry;
pub mod services;
pub mod cleanup;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tweak {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub registry_keys: Vec<RegistryKey>,
    pub services: Vec<String>,
    pub windows_versions: Vec<String>,
    pub revertible: bool,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryKey {
    pub path: String,
    pub value: String,
    pub data: serde_json::Value,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TweakResult {
    pub success: bool,
    pub message: String,
    pub tweak_id: String,
}

pub fn get_all_tweaks() -> Vec<Tweak> {
    vec![
        // Privacy & Telemetry
        Tweak {
            id: "disable-telemetry".to_string(),
            name: "Disable Telemetry".to_string(),
            category: "privacy".to_string(),
            description: "Disable Windows telemetry and data collection services".to_string(),
            registry_keys: vec![
                RegistryKey {
                    path: "HKEY_LOCAL_MACHINE\\SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection".to_string(),
                    value: "AllowDiagnosticData".to_string(),
                    data: serde_json::json!(0),
                    kind: "DWORD".to_string(),
                }
            ],
            services: vec!["DiagTrack".to_string(), "dmwappushservice".to_string()],
            windows_versions: vec!["10".to_string(), "11".to_string()],
            revertible: true,
            enabled: false,
        },
        Tweak {
            id: "disable-cortana".to_string(),
            name: "Disable Cortana".to_string(),
            category: "privacy".to_string(),
            description: "Disable Cortana voice assistant".to_string(),
            registry_keys: vec![
                RegistryKey {
                    path: "HKEY_CURRENT_USER\\Software\\Microsoft\\Personalization\\Settings".to_string(),
                    value: "AcceptedPrivacyPolicy".to_string(),
                    data: serde_json::json!(0),
                    kind: "DWORD".to_string(),
                }
            ],
            services: vec![],
            windows_versions: vec!["10".to_string(), "11".to_string()],
            revertible: true,
            enabled: false,
        },
        // Performance
        Tweak {
            id: "disable-animations".to_string(),
            name: "Disable Animations".to_string(),
            category: "performance".to_string(),
            description: "Disable Windows UI animations for faster response".to_string(),
            registry_keys: vec![
                RegistryKey {
                    path: "HKEY_CURRENT_USER\\Control Panel\\Desktop".to_string(),
                    value: "UserPreferencesMask".to_string(),
                    data: serde_json::json!("90 12 01 80"),
                    kind: "Binary".to_string(),
                }
            ],
            services: vec![],
            windows_versions: vec!["10".to_string(), "11".to_string()],
            revertible: true,
            enabled: false,
        },
        Tweak {
            id: "disable-search-indexing".to_string(),
            name: "Disable Search Indexing".to_string(),
            category: "performance".to_string(),
            description: "Disable Windows Search indexing to free up disk I/O".to_string(),
            registry_keys: vec![],
            services: vec!["WSearch".to_string()],
            windows_versions: vec!["10".to_string(), "11".to_string()],
            revertible: true,
            enabled: false,
        },
        // Gaming
        Tweak {
            id: "enable-game-mode".to_string(),
            name: "Enable Game Mode".to_string(),
            category: "gaming".to_string(),
            description: "Enable Windows Game Mode for better gaming performance".to_string(),
            registry_keys: vec![
                RegistryKey {
                    path: "HKEY_CURRENT_USER\\Software\\Microsoft\\GameBar".to_string(),
                    value: "AllowAutoGameMode".to_string(),
                    data: serde_json::json!(1),
                    kind: "DWORD".to_string(),
                }
            ],
            services: vec![],
            windows_versions: vec!["10".to_string(), "11".to_string()],
            revertible: true,
            enabled: false,
        },
        // Bloatware
        Tweak {
            id: "disable-xbox-services".to_string(),
            name: "Disable Xbox Services".to_string(),
            category: "bloatware".to_string(),
            description: "Disable Xbox Live related services and features".to_string(),
            registry_keys: vec![],
            services: vec!["XboxNetApiSvc".to_string(), "xbgm".to_string()],
            windows_versions: vec!["10".to_string(), "11".to_string()],
            revertible: true,
            enabled: false,
        },
        Tweak {
            id: "disable-onedrive".to_string(),
            name: "Disable OneDrive".to_string(),
            category: "bloatware".to_string(),
            description: "Disable Microsoft OneDrive cloud storage integration".to_string(),
            registry_keys: vec![
                RegistryKey {
                    path: "HKEY_LOCAL_MACHINE\\SOFTWARE\\Policies\\Microsoft\\Windows\\OneDrive".to_string(),
                    value: "DisableFileSyncNGSC".to_string(),
                    data: serde_json::json!(1),
                    kind: "DWORD".to_string(),
                }
            ],
            services: vec!["OneSyncSvc".to_string()],
            windows_versions: vec!["10".to_string(), "11".to_string()],
            revertible: true,
            enabled: false,
        },
        // System
        Tweak {
            id: "enable-long-paths".to_string(),
            name: "Enable Long Paths".to_string(),
            category: "system".to_string(),
            description: "Enable support for file paths longer than 260 characters".to_string(),
            registry_keys: vec![
                RegistryKey {
                    path: "HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Control\\FileSystem".to_string(),
                    value: "LongPathsEnabled".to_string(),
                    data: serde_json::json!(1),
                    kind: "DWORD".to_string(),
                }
            ],
            services: vec![],
            windows_versions: vec!["10".to_string(), "11".to_string()],
            revertible: true,
            enabled: false,
        },
    ]
}
