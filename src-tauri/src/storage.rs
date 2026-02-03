use image::RgbaImage;
use std::path::PathBuf;

pub fn get_screenshot_dir(custom_path: Option<&str>) -> Result<PathBuf, String> {
    let path = match custom_path {
        Some(p) if !p.is_empty() => PathBuf::from(p),
        _ => {
            let mut path = dirs::picture_dir().ok_or("Could not find Pictures directory")?;
            path.push("Screenshots");
            path
        }
    };

    std::fs::create_dir_all(&path)
        .map_err(|e| format!("Failed to create screenshots dir: {e}"))?;
    Ok(path)
}

pub fn save_screenshot(img: &RgbaImage, custom_path: Option<&str>) -> Result<PathBuf, String> {
    let dir = get_screenshot_dir(custom_path)?;
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let path = dir.join(format!("Screenshot_{timestamp}.png"));

    img.save(&path)
        .map_err(|e| format!("Failed to save screenshot: {e}"))?;

    Ok(path)
}
