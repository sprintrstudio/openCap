use image::RgbaImage;
use screenshots::Screen;

pub fn capture_full_screen() -> Result<RgbaImage, String> {
    let screens = Screen::all().map_err(|e| format!("Failed to enumerate screens: {e}"))?;
    let primary = screens
        .into_iter()
        .find(|s| s.display_info.is_primary)
        .ok_or("No primary screen found")?;

    let img = primary
        .capture()
        .map_err(|e| format!("Failed to capture screen: {e}"))?;

    let rgba = RgbaImage::from_raw(img.width(), img.height(), img.into_raw())
        .ok_or("Failed to convert capture to image")?;

    Ok(rgba)
}

pub fn crop_region(img: &RgbaImage, x: u32, y: u32, w: u32, h: u32) -> Result<RgbaImage, String> {
    if x + w > img.width() || y + h > img.height() {
        return Err("Region extends beyond image bounds".into());
    }
    if w == 0 || h == 0 {
        return Err("Region has zero size".into());
    }
    let cropped = image::imageops::crop_imm(img, x, y, w, h).to_image();
    Ok(cropped)
}

pub fn image_to_base64_png(img: &RgbaImage) -> Result<String, String> {
    use base64::Engine;
    use image::ImageEncoder;
    use std::io::Cursor;

    let mut buf = Cursor::new(Vec::new());
    image::codecs::png::PngEncoder::new(&mut buf)
        .write_image(img.as_raw(), img.width(), img.height(), image::ExtendedColorType::Rgba8)
        .map_err(|e| format!("PNG encode failed: {e}"))?;

    let b64 = base64::engine::general_purpose::STANDARD.encode(buf.into_inner());
    Ok(format!("data:image/png;base64,{b64}"))
}
