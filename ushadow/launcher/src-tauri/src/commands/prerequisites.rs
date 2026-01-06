use std::process::Command;
use crate::models::PrerequisiteStatus;

/// Check if Docker is installed and running
pub fn check_docker() -> (bool, bool, Option<String>) {
    let version_output = Command::new("docker").args(["--version"]).output();

    let (installed, version) = match version_output {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            (true, Some(version))
        }
        _ => (false, None),
    };

    if !installed {
        return (false, false, None);
    }

    let info_output = Command::new("docker").args(["info"]).output();
    let running = matches!(info_output, Ok(output) if output.status.success());

    (installed, running, version)
}

/// Check if Tailscale is installed and connected
pub fn check_tailscale() -> (bool, bool, Option<String>) {
    let version_output = Command::new("tailscale").args(["--version"]).output();

    let (installed, version) = match version_output {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout)
                .lines()
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
            (true, Some(version))
        }
        _ => (false, None),
    };

    if !installed {
        return (false, false, None);
    }

    let status_output = Command::new("tailscale").args(["status"]).output();
    let connected = matches!(status_output, Ok(output) if output.status.success());

    (installed, connected, version)
}

/// Get full prerequisite status
#[tauri::command]
pub fn check_prerequisites() -> Result<PrerequisiteStatus, String> {
    let (docker_installed, docker_running, docker_version) = check_docker();
    let (tailscale_installed, tailscale_connected, tailscale_version) = check_tailscale();

    Ok(PrerequisiteStatus {
        docker_installed,
        docker_running,
        tailscale_installed,
        tailscale_connected,
        docker_version,
        tailscale_version,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_docker_returns_tuple() {
        let (installed, running, version) = check_docker();
        // Just verify it returns without panicking
        // Actual values depend on system state
        if installed {
            assert!(version.is_some());
        }
        println!("Docker: installed={}, running={}, version={:?}", installed, running, version);
    }

    #[test]
    fn test_check_tailscale_returns_tuple() {
        let (installed, connected, version) = check_tailscale();
        if installed {
            assert!(version.is_some());
        }
        println!("Tailscale: installed={}, connected={}, version={:?}", installed, connected, version);
    }

    #[test]
    fn test_check_prerequisites_returns_status() {
        let result = check_prerequisites();
        assert!(result.is_ok());
        let status = result.unwrap();
        println!("Prerequisites: docker={}/{}, tailscale={}/{}",
            status.docker_installed, status.docker_running,
            status.tailscale_installed, status.tailscale_connected);
    }
}
