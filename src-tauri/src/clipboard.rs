use arboard::{Clipboard, ImageData};
use image::RgbaImage;

pub fn copy_image_to_clipboard(img: &RgbaImage) -> Result<(), String> {
    let mut clipboard = Clipboard::new().map_err(|e| format!("Clipboard error: {e}"))?;
    let data = ImageData {
        width: img.width() as usize,
        height: img.height() as usize,
        bytes: std::borrow::Cow::Borrowed(img.as_raw()),
    };
    clipboard
        .set_image(data)
        .map_err(|e| format!("Failed to copy to clipboard: {e}"))?;
    Ok(())
}
