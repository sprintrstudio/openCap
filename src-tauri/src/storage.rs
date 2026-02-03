use image::RgbaImage;
use std::path::PathBuf;

pub fn get_screenshot_dir() -> Result<PathBuf, String> {
    let mut path = dirs::picture_dir().ok_or("Could not find Pictures directory")?;
    path.push("Screenshots");
    std::fs::create_dir_all(&path)
        .map_err(|e| format!("Failed to create screenshots dir: {e}"))?;
    Ok(path)
}

pub fn save_screenshot(img: &RgbaImage) -> Result<PathBuf, String> {
    let dir = get_screenshot_dir()?;
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let path = dir.join(format!("Screenshot_{timestamp}.png"));

    img.save(&path)
        .map_err(|e| format!("Failed to save screenshot: {e}"))?;

    Ok(path)
}
