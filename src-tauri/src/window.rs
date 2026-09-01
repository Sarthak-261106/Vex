use serde::{Deserialize, Serialize};
use std::fs;
use tauri::{AppHandle, Manager, Result};

#[derive(Deserialize, Serialize)]
struct WindowPlacement { x: i32, y: i32 }

fn placement_path(app: &AppHandle) -> Option<std::path::PathBuf> {
    app.path().app_config_dir().ok().map(|path| path.join("window-placement.json"))
}

pub fn restore_main_window(app: &AppHandle) -> Result<()> {
    let window = app.get_webview_window("main").expect("main VEX window");
    window.set_always_on_top(true)?;
    if let Some(path) = placement_path(app) {
        if let Ok(contents) = fs::read_to_string(path) {
            if let Ok(placement) = serde_json::from_str::<WindowPlacement>(&contents) {
                let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(placement.x, placement.y)));
            }
        }
    }
    Ok(())
}

#[tauri::command]
pub fn save_window_position(app: AppHandle, x: i32, y: i32) -> Result<(), String> {
    let path = placement_path(&app).ok_or("Unable to locate VEX configuration folder")?;
    if let Some(parent) = path.parent() { fs::create_dir_all(parent).map_err(|error| error.to_string())?; }
    let contents = serde_json::to_string(&WindowPlacement { x, y }).map_err(|error| error.to_string())?;
    fs::write(path, contents).map_err(|error| error.to_string())
}
