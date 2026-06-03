use std::process::Command;

pub fn disable_service(service_name: &str) -> Result<(), String> {
    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            &format!("Stop-Service -Name {} -Force -ErrorAction SilentlyContinue; Set-Service -Name {} -StartupType Disabled -ErrorAction SilentlyContinue", service_name, service_name),
        ])
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to disable service: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

pub fn enable_service(service_name: &str) -> Result<(), String> {
    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            &format!(
                "Set-Service -Name {} -StartupType Automatic -ErrorAction SilentlyContinue; Start-Service -Name {} -ErrorAction SilentlyContinue",
                service_name, service_name
            ),
        ])
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to enable service: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
