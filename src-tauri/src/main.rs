#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tray;
mod window;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![window::save_window_position])
        .setup(|app| {
            // Explicitly no startup registration, service, watcher, or auto-relaunch.
            window::restore_main_window(app.handle())?;
            tray::build(app.handle())?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                // Closing VEX exits the process; it never hides to a background process.
                window.app_handle().exit(0);
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running VEX");
}
