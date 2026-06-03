use winreg::enums::*;
use winreg::RegKey;

pub fn set_registry_value(path: &str, value_name: &str, value: &str, value_type: &str) -> Result<(), String> {
    let parts: Vec<&str> = path.split('\\').collect();
    if parts.is_empty() {
        return Err("Invalid registry path".to_string());
    }

    let root = match parts[0] {
        "HKEY_LOCAL_MACHINE" => RegKey::predef(HKEY_LOCAL_MACHINE),
        "HKEY_CURRENT_USER" => RegKey::predef(HKEY_CURRENT_USER),
        _ => return Err("Unknown registry hive".to_string()),
    };

    let subpath = parts[1..].join("\\");
    let (key, _) = root
        .create_subkey(&subpath)
        .map_err(|e| format!("Failed to access registry: {}", e))?;

    match value_type {
        "DWORD" => {
            let dword_value: u32 = value
                .parse()
                .map_err(|_| format!("Invalid DWORD value: {}", value))?;
            key.set_value(value_name, &dword_value)
                .map_err(|e| format!("Failed to set registry value: {}", e))?;
        }
        "String" => {
            key.set_value(value_name, &value)
                .map_err(|e| format!("Failed to set registry value: {}", e))?;
        }
        _ => return Err(format!("Unknown registry type: {}", value_type)),
    }

    Ok(())
}

pub fn get_registry_value(path: &str, value_name: &str) -> Result<String, String> {
    let parts: Vec<&str> = path.split('\\').collect();
    if parts.is_empty() {
        return Err("Invalid registry path".to_string());
    }

    let root = match parts[0] {
        "HKEY_LOCAL_MACHINE" => RegKey::predef(HKEY_LOCAL_MACHINE),
        "HKEY_CURRENT_USER" => RegKey::predef(HKEY_CURRENT_USER),
        _ => return Err("Unknown registry hive".to_string()),
    };

    let subpath = parts[1..].join("\\");
    let key = root
        .open_subkey(&subpath)
        .map_err(|e| format!("Failed to open registry key: {}", e))?;

    let value: String = key
        .get_value(value_name)
        .map_err(|e| format!("Failed to get registry value: {}", e))?;

    Ok(value)
}
