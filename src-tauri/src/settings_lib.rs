use crate::config::{self, Config};
use crate::programs::{self, ImageProgram};
use tauri::WebviewWindowBuilder;

#[tauri::command]
pub fn get_config() -> Config {
    config::load_config()
}

#[tauri::command]
pub fn save_config_cmd(config: Config) -> Result<(), String> {
    // Validate: at least one output option must be enabled
    if !config.copy_to_clipboard && !config.auto_open && !config.save_locally {
        return Err("At least one option must be enabled (clipboard, auto-open, or save locally)".to_string());
    }
    config::save_config(&config)
}

#[tauri::command]
pub fn get_image_programs() -> Vec<ImageProgram> {
    programs::detect_image_programs()
}

#[tauri::command]
pub fn browse_folder(_current_path: Option<String>) -> Option<String> {
    use std::process::Command;

    #[cfg(target_os = "windows")]
    {
        // Use PowerShell to show folder picker dialog
        let script = r#"
            Add-Type -AssemblyName System.Windows.Forms
            $dialog = New-Object System.Windows.Forms.FolderBrowserDialog
            $dialog.Description = "Select screenshot save location"
            $dialog.ShowNewFolderButton = $true
            if ($dialog.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {
                Write-Output $dialog.SelectedPath
            }
        "#;

        let output = Command::new("powershell")
            .args(["-NoProfile", "-Command", script])
            .output()
            .ok()?;

        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Some(path);
            }
        }
        None
    }

    #[cfg(target_os = "macos")]
    {
        let output = Command::new("osascript")
            .args(["-e", "POSIX path of (choose folder with prompt \"Select screenshot save location\")"])
            .output()
            .ok()?;

        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Some(path);
            }
        }
        None
    }

    #[cfg(target_os = "linux")]
    {
        // Try zenity first, then kdialog
        let output = Command::new("zenity")
            .args(["--file-selection", "--directory", "--title=Select screenshot save location"])
            .output()
            .or_else(|_| {
                Command::new("kdialog")
                    .args(["--getexistingdirectory", _current_path.as_deref().unwrap_or("~")])
                    .output()
            })
            .ok()?;

        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Some(path);
            }
        }
        None
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        let _ = _current_path;
        None
    }
}

#[tauri::command]
pub fn get_default_save_path() -> Result<String, String> {
    let mut path = dirs::picture_dir().ok_or("Could not find Pictures directory")?;
    path.push("Screenshots");
    path.to_str()
        .map(|s| s.to_string())
        .ok_or("Invalid path".to_string())
}

pub fn run_settings() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config_cmd,
            get_image_programs,
            browse_folder,
            get_default_save_path,
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            WebviewWindowBuilder::new(app, "settings", tauri::WebviewUrl::App("settings.html".into()))
                .title("OpenCap Settings")
                .inner_size(450.0, 520.0)
                .resizable(false)
                .center()
                .build()
                .map_err(|e| format!("Window creation failed: {e}"))?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running settings application");
}
