mod capture;
mod clipboard;
mod storage;

use image::RgbaImage;
use std::sync::Mutex;
use tauri::Manager;
use tauri::WebviewWindowBuilder;

pub struct PendingCapture(pub Mutex<Option<RgbaImage>>);
pub struct PendingDataUrl(pub Mutex<Option<String>>);

#[tauri::command]
fn get_pending_data_url(state: tauri::State<PendingDataUrl>) -> Result<String, String> {
    state.0.lock().unwrap().take().ok_or("No pending screenshot".into())
}

#[tauri::command]
fn finish_region_capture(
    app: tauri::AppHandle,
    state: tauri::State<PendingCapture>,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
) -> Result<(), String> {
    let img = state.0.lock().unwrap().take().ok_or("No pending capture")?;
    let cropped = capture::crop_region(&img, x, y, w, h)?;

    let path = storage::save_screenshot(&cropped).map_err(|e| format!("Save failed: {e}"))?;
    if let Err(e) = clipboard::copy_image_to_clipboard(&cropped) {
        log::error!("Clipboard failed: {e}");
    }

    let _ = open::that(&path);
    app.exit(0);
    Ok(())
}

#[tauri::command]
fn capture_full_and_finish(
    app: tauri::AppHandle,
    state: tauri::State<PendingCapture>,
) -> Result<(), String> {
    let img = state.0.lock().unwrap().take().ok_or("No pending capture")?;

    let path = storage::save_screenshot(&img).map_err(|e| format!("Save failed: {e}"))?;
    if let Err(e) = clipboard::copy_image_to_clipboard(&img) {
        log::error!("Clipboard failed: {e}");
    }

    let _ = open::that(&path);
    app.exit(0);
    Ok(())
}

#[tauri::command]
fn cancel_region_capture(app: tauri::AppHandle) -> Result<(), String> {
    app.exit(0);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(PendingCapture(Mutex::new(None)))
        .manage(PendingDataUrl(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            get_pending_data_url,
            finish_region_capture,
            capture_full_and_finish,
            cancel_region_capture,
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Capture BEFORE any window exists — guaranteed clean
            let img = capture::capture_full_screen()?;
            let data_url = capture::image_to_base64_png(&img)?;

            *app.state::<PendingCapture>().0.lock().unwrap() = Some(img);
            *app.state::<PendingDataUrl>().0.lock().unwrap() = Some(data_url);

            // NOW create the overlay window
            WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::App("index.html".into()))
                .fullscreen(true)
                .decorations(false)
                .always_on_top(true)
                .transparent(true)
                .title("OpenCap")
                .build()
                .map_err(|e| format!("Window creation failed: {e}"))?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
