mod capture;
mod clipboard;
mod storage;
mod tray;

use image::RgbaImage;
use std::sync::Mutex;
use tauri::Manager;

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

    match storage::save_screenshot(&cropped) {
        Ok(path) => log::info!("Region screenshot saved to {}", path.display()),
        Err(e) => log::error!("Save failed: {e}"),
    }
    if let Err(e) = clipboard::copy_image_to_clipboard(&cropped) {
        log::error!("Clipboard failed: {e}");
    }

    use tauri_plugin_notification::NotificationExt;
    let _ = app.notification().builder().title("OpenCap").body("Region captured!").show();

    if let Some(win) = app.get_webview_window("overlay") {
        let _ = win.close();
    }
    Ok(())
}

#[tauri::command]
fn cancel_region_capture(app: tauri::AppHandle, state: tauri::State<PendingCapture>) -> Result<(), String> {
    *state.0.lock().unwrap() = None;
    if let Some(win) = app.get_webview_window("overlay") {
        let _ = win.close();
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .manage(PendingCapture(Mutex::new(None)))
        .manage(PendingDataUrl(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            get_pending_data_url,
            finish_region_capture,
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

            tray::setup_tray(app.handle())?;

            if let Some(window) = app.get_webview_window("main") {
                let _ = window.hide();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
