use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};
use tauri_plugin_notification::NotificationExt;

use crate::{capture, clipboard, storage};

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let capture_full_i = MenuItem::with_id(app, "capture_full", "Capture Full Screen", true, None::<&str>)?;
    let capture_region_i = MenuItem::with_id(app, "capture_region", "Capture Region", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&capture_full_i, &capture_region_i, &quit_i])?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .tooltip("OpenCap")
        .on_menu_event(|app, event| match event.id.as_ref() {
            "capture_full" => {
                do_full_capture(app);
            }
            "capture_region" => {
                start_region_overlay(app);
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}

fn do_full_capture(app: &AppHandle) {
    match capture::capture_full_screen() {
        Ok(img) => {
            match storage::save_screenshot(&img) {
                Ok(path) => log::info!("Screenshot saved to {}", path.display()),
                Err(e) => log::error!("Save failed: {e}"),
            }
            if let Err(e) = clipboard::copy_image_to_clipboard(&img) {
                log::error!("Clipboard failed: {e}");
            }
            let _ = app.notification()
                .builder()
                .title("OpenCap")
                .body("Screenshot captured!")
                .show();
        }
        Err(e) => {
            log::error!("Capture failed: {e}");
        }
    }
}

pub fn start_region_overlay(app: &AppHandle) {
    // Capture screen first, store in state, then open overlay
    match capture::capture_full_screen() {
        Ok(img) => {
            let data_url = match capture::image_to_base64_png(&img) {
                Ok(d) => d,
                Err(e) => {
                    log::error!("Base64 encode failed: {e}");
                    return;
                }
            };

            // Store pending capture
            let state = app.state::<crate::PendingCapture>();
            *state.0.lock().unwrap() = Some(img);

            // Store the data URL so the overlay can retrieve it
            let data_state = app.state::<crate::PendingDataUrl>();
            *data_state.0.lock().unwrap() = Some(data_url);

            // Open overlay window
            if let Err(e) = tauri::WebviewWindowBuilder::new(
                app,
                "overlay",
                tauri::WebviewUrl::App("index.html?mode=overlay".into()),
            )
            .title("Region Select")
            .fullscreen(true)
            .decorations(false)
            .always_on_top(true)
            .transparent(true)
            .build()
            {
                log::error!("Failed to create overlay: {e}");
            }
        }
        Err(e) => {
            log::error!("Capture failed: {e}");
        }
    }
}
