mod capture;
mod clipboard;
mod storage;

use image::RgbaImage;
use std::sync::Mutex;
use tauri::Manager;
use tauri::WebviewWindowBuilder;

pub struct PendingCapture(pub Mutex<Option<RgbaImage>>);
pub struct PendingDataUrl(pub Mutex<Option<String>>);

#[derive(Clone, serde::Serialize)]
pub struct ScreenLayout {
    pub monitors: Vec<capture::MonitorInfo>,
    pub origin_x: i32,
    pub origin_y: i32,
    pub virtual_width: u32,
    pub virtual_height: u32,
}

pub struct PendingScreenLayout(pub Mutex<Option<ScreenLayout>>);

#[tauri::command]
fn get_pending_data_url(state: tauri::State<PendingDataUrl>) -> Result<String, String> {
    state.0.lock().unwrap().take().ok_or("No pending screenshot".into())
}

#[tauri::command]
fn get_screen_layout(state: tauri::State<PendingScreenLayout>) -> Result<ScreenLayout, String> {
    state
        .0
        .lock()
        .unwrap()
        .clone()
        .ok_or("No screen layout".into())
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
fn finish_monitor_capture(
    app: tauri::AppHandle,
    capture_state: tauri::State<PendingCapture>,
    layout_state: tauri::State<PendingScreenLayout>,
    monitor_index: usize,
) -> Result<(), String> {
    let img = capture_state
        .0
        .lock()
        .unwrap()
        .take()
        .ok_or("No pending capture")?;
    let layout = layout_state
        .0
        .lock()
        .unwrap()
        .clone()
        .ok_or("No screen layout")?;
    let mon = layout
        .monitors
        .get(monitor_index)
        .ok_or("Invalid monitor index")?;

    let x = (mon.x - layout.origin_x) as u32;
    let y = (mon.y - layout.origin_y) as u32;
    let cropped = capture::crop_region(&img, x, y, mon.width, mon.height)?;

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
        .manage(PendingScreenLayout(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            get_pending_data_url,
            get_screen_layout,
            finish_region_capture,
            finish_monitor_capture,
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

            // Capture all screens before any window exists — guaranteed clean
            let composite = capture::capture_all_screens()?;
            let data_url = capture::image_to_base64_png(&composite.image)?;

            let layout = ScreenLayout {
                monitors: composite.monitors.clone(),
                origin_x: composite.origin_x,
                origin_y: composite.origin_y,
                virtual_width: composite.virtual_width,
                virtual_height: composite.virtual_height,
            };

            *app.state::<PendingCapture>().0.lock().unwrap() = Some(composite.image);
            *app.state::<PendingDataUrl>().0.lock().unwrap() = Some(data_url);
            *app.state::<PendingScreenLayout>().0.lock().unwrap() = Some(layout);

            // Create overlay window spanning the entire virtual desktop
            WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::App("index.html".into()))
                .position(composite.origin_x as f64, composite.origin_y as f64)
                .inner_size(composite.virtual_width as f64, composite.virtual_height as f64)
                .decorations(false)
                .always_on_top(true)
                .transparent(true)
                .skip_taskbar(true)
                .title("OpenCap")
                .build()
                .map_err(|e| format!("Window creation failed: {e}"))?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
