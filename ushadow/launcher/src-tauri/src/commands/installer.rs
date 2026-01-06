use super::utils::silent_command;
use std::process::Command;

/// Check if Homebrew is installed (macOS)
#[cfg(target_os = "macos")]
pub fn check_brew_installed() -> bool {
    silent_command("brew").args(["--version"]).output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

/// Install Docker via Homebrew (macOS)
#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn install_docker_via_brew() -> Result<String, String> {
    if !check_brew_installed() {
        return Err("Homebrew is not installed. Please install from https://brew.sh".to_string());
    }

    // Run brew install --cask docker
    let output = Command::new("brew")
        .args(["install", "--cask", "docker"])
        .output()
        .map_err(|e| format!("Failed to run brew: {}", e))?;

    if output.status.success() {
        Ok("Docker Desktop installed successfully via Homebrew".to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Brew install failed: {}", stderr))
    }
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub async fn install_docker_via_brew() -> Result<String, String> {
    Err("Homebrew installation is only available on macOS".to_string())
}

/// Start Docker Desktop (macOS)
#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn start_docker_desktop_macos() -> Result<String, String> {
    let output = Command::new("open")
        .args(["-a", "Docker"])
        .output()
        .map_err(|e| format!("Failed to open Docker Desktop: {}", e))?;

    if output.status.success() {
        Ok("Docker Desktop starting...".to_string())
    } else {
        Err("Failed to start Docker Desktop".to_string())
    }
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub async fn start_docker_desktop_macos() -> Result<String, String> {
    Err("macOS Docker Desktop start is only available on macOS".to_string())
}

/// Start Docker Desktop (Windows)
#[cfg(target_os = "windows")]
#[tauri::command]
pub async fn start_docker_desktop_windows() -> Result<String, String> {
    use std::path::Path;

    // Try common Docker Desktop installation paths
    let paths = vec![
        r"C:\Program Files\Docker\Docker\Docker Desktop.exe",
        r"C:\Program Files\Docker\Docker Desktop.exe",
    ];

    for path in paths {
        if Path::new(path).exists() {
            let output = Command::new(path)
                .spawn()
                .map_err(|e| format!("Failed to start Docker Desktop: {}", e))?;

            return Ok("Docker Desktop starting...".to_string());
        }
    }

    Err("Docker Desktop.exe not found in expected locations".to_string())
}

#[cfg(not(target_os = "windows"))]
#[tauri::command]
pub async fn start_docker_desktop_windows() -> Result<String, String> {
    Err("Windows Docker Desktop start is only available on Windows".to_string())
}

/// Start Docker service (Linux)
#[cfg(target_os = "linux")]
#[tauri::command]
pub async fn start_docker_service_linux() -> Result<String, String> {
    // Try systemctl first (most common)
    let systemctl_output = Command::new("systemctl")
        .args(["start", "docker"])
        .output();

    if let Ok(output) = systemctl_output {
        if output.status.success() {
            return Ok("Docker service started via systemctl".to_string());
        }
    }

    // Fallback to service command
    let service_output = Command::new("service")
        .args(["docker", "start"])
        .output();

    if let Ok(output) = service_output {
        if output.status.success() {
            return Ok("Docker service started via service command".to_string());
        }
    }

    Err("Failed to start Docker service. Try: sudo systemctl start docker".to_string())
}

#[cfg(not(target_os = "linux"))]
#[tauri::command]
pub async fn start_docker_service_linux() -> Result<String, String> {
    Err("Linux Docker service start is only available on Linux".to_string())
}
