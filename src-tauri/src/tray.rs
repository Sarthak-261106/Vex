use tauri::{menu::{Menu, MenuItem}, tray::TrayIconBuilder, AppHandle, Manager, Result};

pub fn build(app: &AppHandle) -> Result<()> {
    let show = MenuItem::with_id(app, "show", "Show VEX", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "hide", "Hide VEX", true, None::<&str>)?;
    let mute = MenuItem::with_id(app, "mute", "Mute", true, None::<&str>)?;
    let blind = MenuItem::with_id(app, "blind", "Blind", true, None::<&str>)?;
    let fullscreen = MenuItem::with_id(app, "fullscreen", "Full Screen", false, None::<&str>)?;
    let order = MenuItem::with_id(app, "order", "Give Order", false, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "Settings", false, None::<&str>)?;
    let permissions = MenuItem::with_id(app, "permissions", "Permissions", false, None::<&str>)?;
    let memory = MenuItem::with_id(app, "memory", "Memory", false, None::<&str>)?;
    let privacy = MenuItem::with_id(app, "privacy", "Privacy", false, None::<&str>)?;
    let about = MenuItem::with_id(app, "about", "About", true, None::<&str>)?;
    let exit = MenuItem::with_id(app, "exit", "Exit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &hide, &mute, &blind, &fullscreen, &order, &settings, &permissions, &memory, &privacy, &about, &exit])?;
    TrayIconBuilder::with_id("vex-tray")
        .menu(&menu)
        .tooltip("VEX — The Spirit of Your Laptop")
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "show" => { if let Some(w) = app.get_webview_window("main") { let _ = w.show(); let _ = w.set_focus(); } }
            "hide" => { if let Some(w) = app.get_webview_window("main") { let _ = w.hide(); } }
            "exit" => app.exit(0),
            _ => { if let Some(w) = app.get_webview_window("main") { let _ = w.show(); } }
        })
        .build(app)?;
    Ok(())
}
