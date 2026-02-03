use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageProgram {
    pub name: String,
    pub path: String,
}

/// Detects installed image editing/viewing programs on the system
pub fn detect_image_programs() -> Vec<ImageProgram> {
    let mut programs = vec![
        ImageProgram {
            name: "System Default".to_string(),
            path: "default".to_string(),
        },
    ];

    #[cfg(target_os = "windows")]
    {
        programs.extend(detect_windows_programs());
    }

    #[cfg(target_os = "macos")]
    {
        programs.extend(detect_macos_programs());
    }

    #[cfg(target_os = "linux")]
    {
        programs.extend(detect_linux_programs());
    }

    programs
}

#[cfg(target_os = "windows")]
fn detect_windows_programs() -> Vec<ImageProgram> {
    let mut found = Vec::new();

    // Common Windows image programs and their typical paths
    let candidates = [
        ("Paint", r"C:\Windows\System32\mspaint.exe"),
        ("Paint 3D", r"C:\Program Files\WindowsApps\Microsoft.MSPaint_*\PaintStudio.View.exe"),
        ("Photos", r"C:\Program Files\WindowsApps\Microsoft.Windows.Photos_*\Microsoft.Photos.exe"),
        ("GIMP", r"C:\Program Files\GIMP 2\bin\gimp-2.10.exe"),
        ("GIMP", r"C:\Program Files\GIMP 2\bin\gimp-2.99.exe"),
        ("Photoshop", r"C:\Program Files\Adobe\Adobe Photoshop 2024\Photoshop.exe"),
        ("Photoshop", r"C:\Program Files\Adobe\Adobe Photoshop 2023\Photoshop.exe"),
        ("Photoshop", r"C:\Program Files\Adobe\Adobe Photoshop CC 2019\Photoshop.exe"),
        ("Krita", r"C:\Program Files\Krita (x64)\bin\krita.exe"),
        ("IrfanView", r"C:\Program Files\IrfanView\i_view64.exe"),
        ("IrfanView", r"C:\Program Files (x86)\IrfanView\i_view32.exe"),
        ("XnView", r"C:\Program Files\XnView\xnview.exe"),
        ("FastStone", r"C:\Program Files (x86)\FastStone Image Viewer\FSViewer.exe"),
        ("Paint.NET", r"C:\Program Files\paint.net\paintdotnet.exe"),
        ("Affinity Photo", r"C:\Program Files\Affinity\Photo 2\Photo.exe"),
        ("Affinity Photo", r"C:\Program Files\Affinity\Photo\Photo.exe"),
    ];

    for (name, path_pattern) in candidates {
        // Handle glob patterns for Windows Store apps
        if path_pattern.contains('*') {
            if let Some(resolved) = resolve_glob_path(path_pattern) {
                found.push(ImageProgram {
                    name: name.to_string(),
                    path: resolved,
                });
            }
        } else {
            let path = PathBuf::from(path_pattern);
            if path.exists() {
                // Avoid duplicates (e.g., multiple Photoshop versions)
                if !found.iter().any(|p| p.name == name) {
                    found.push(ImageProgram {
                        name: name.to_string(),
                        path: path_pattern.to_string(),
                    });
                }
            }
        }
    }

    found
}

#[cfg(target_os = "windows")]
fn resolve_glob_path(pattern: &str) -> Option<String> {
    // Simple glob resolution for Windows Store app paths
    let parts: Vec<&str> = pattern.split('*').collect();
    if parts.len() != 2 {
        return None;
    }

    let parent = PathBuf::from(parts[0].trim_end_matches('_'));
    let parent_dir = parent.parent()?;
    let prefix = parent.file_name()?.to_str()?;
    let suffix = parts[1];

    if let Ok(entries) = std::fs::read_dir(parent_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_str()?;
            if name_str.starts_with(prefix) {
                let full_path = entry.path().join(suffix.trim_start_matches('\\'));
                if full_path.exists() {
                    return full_path.to_str().map(|s| s.to_string());
                }
            }
        }
    }

    None
}

#[cfg(target_os = "macos")]
fn detect_macos_programs() -> Vec<ImageProgram> {
    let mut found = Vec::new();

    let candidates = [
        ("Preview", "/System/Applications/Preview.app"),
        ("Photos", "/System/Applications/Photos.app"),
        ("GIMP", "/Applications/GIMP-2.10.app"),
        ("Photoshop", "/Applications/Adobe Photoshop 2024/Adobe Photoshop 2024.app"),
        ("Krita", "/Applications/krita.app"),
        ("Affinity Photo", "/Applications/Affinity Photo 2.app"),
        ("Pixelmator Pro", "/Applications/Pixelmator Pro.app"),
    ];

    for (name, path) in candidates {
        if PathBuf::from(path).exists() {
            found.push(ImageProgram {
                name: name.to_string(),
                path: path.to_string(),
            });
        }
    }

    found
}

#[cfg(target_os = "linux")]
fn detect_linux_programs() -> Vec<ImageProgram> {
    let mut found = Vec::new();

    // Check common Linux image programs via `which`
    let candidates = [
        ("GIMP", "gimp"),
        ("Krita", "krita"),
        ("Inkscape", "inkscape"),
        ("ImageMagick", "display"),
        ("Eye of GNOME", "eog"),
        ("Gwenview", "gwenview"),
        ("feh", "feh"),
        ("Shotwell", "shotwell"),
    ];

    for (name, cmd) in candidates {
        if let Ok(output) = std::process::Command::new("which").arg(cmd).output() {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path.is_empty() {
                    found.push(ImageProgram {
                        name: name.to_string(),
                        path,
                    });
                }
            }
        }
    }

    found
}
