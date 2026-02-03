use image::RgbaImage;
use screenshots::Screen;

#[derive(Clone, serde::Serialize)]
pub struct MonitorInfo {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f32,
}

pub struct CompositeCapture {
    pub image: RgbaImage,
    pub monitors: Vec<MonitorInfo>,
    pub origin_x: i32,
    pub origin_y: i32,
    pub virtual_width: u32,
    pub virtual_height: u32,
}

/// Capture all screens and composite them into a single image in logical pixel space.
pub fn capture_all_screens() -> Result<CompositeCapture, String> {
    let screens = Screen::all().map_err(|e| format!("Failed to enumerate screens: {e}"))?;
    if screens.is_empty() {
        return Err("No screens found".into());
    }

    // Capture each screen and collect display info
    let mut captures: Vec<(MonitorInfo, RgbaImage)> = Vec::new();
    for screen in &screens {
        let img = screen
            .capture()
            .map_err(|e| format!("Failed to capture screen: {e}"))?;
        let rgba = RgbaImage::from_raw(img.width(), img.height(), img.into_raw())
            .ok_or("Failed to convert capture to image")?;
        let info = &screen.display_info;
        captures.push((
            MonitorInfo {
                x: info.x,
                y: info.y,
                width: info.width,
                height: info.height,
                scale_factor: info.scale_factor,
            },
            rgba,
        ));
    }

    // Compute virtual desktop bounding box in logical pixels
    let min_x = captures.iter().map(|(d, _)| d.x).min().unwrap();
    let min_y = captures.iter().map(|(d, _)| d.y).min().unwrap();
    let max_x = captures
        .iter()
        .map(|(d, _)| d.x + d.width as i32)
        .max()
        .unwrap();
    let max_y = captures
        .iter()
        .map(|(d, _)| d.y + d.height as i32)
        .max()
        .unwrap();

    let vw = (max_x - min_x) as u32;
    let vh = (max_y - min_y) as u32;

    // Create composite canvas in logical pixel space
    let mut composite = RgbaImage::new(vw, vh);

    let mut monitors = Vec::new();

    for (info, img) in &captures {
        // Resize capture from physical to logical pixels
        let resized = image::imageops::resize(
            img,
            info.width,
            info.height,
            image::imageops::FilterType::Lanczos3,
        );

        let px = (info.x - min_x) as i64;
        let py = (info.y - min_y) as i64;
        image::imageops::overlay(&mut composite, &resized, px, py);

        monitors.push(info.clone());
    }

    Ok(CompositeCapture {
        image: composite,
        monitors,
        origin_x: min_x,
        origin_y: min_y,
        virtual_width: vw,
        virtual_height: vh,
    })
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
        .write_image(
            img.as_raw(),
            img.width(),
            img.height(),
            image::ExtendedColorType::Rgba8,
        )
        .map_err(|e| format!("PNG encode failed: {e}"))?;

    let b64 = base64::engine::general_purpose::STANDARD.encode(buf.into_inner());
    Ok(format!("data:image/png;base64,{b64}"))
}
