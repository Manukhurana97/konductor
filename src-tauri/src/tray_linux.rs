use tauri::{AppHandle, Manager};
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIcon, TrayIconAttributes, TrayIconEvent,
};

pub fn create_system_tray() -> TrayIcon {
    let show_item = MenuItem::new("Show Konductor", true, None);
    let hide_item = MenuItem::new("Hide Konductor", true, None);
    let quit_item = MenuItem::new("Quit", true, None);

    let tray_menu = Menu::new();
    tray_menu.append(&show_item).unwrap();
    tray_menu.append(&hide_item).unwrap();
    tray_menu.append(&PredefinedMenuItem::separator()).unwrap();
    tray_menu.append(&quit_item).unwrap();

    // Try multiple icon paths for different scenarios
    let icon_paths = [
        "/usr/share/icons/hicolor/32x32/apps/konductor.png", // Installed system path
        "src-tauri/icons/32x32.png", // Development path
        "icons/32x32.png", // Alternative development path
        "src-tauri/icons/icon.png", // Fallback to main icon
    ];

    let icon = {
        let mut found_icon = None;
        for path in &icon_paths {
            if let Ok(img) = image::open(path) {
                let rgba8 = img.into_rgba8();
                let (width, height) = rgba8.dimensions();
                let rgba = rgba8.into_raw();
                if let Ok(icon) = tray_icon::Icon::from_rgba(rgba, width, height) {
                    found_icon = Some(icon);
                    break;
                }
            }
        }
        
        // Fallback: create a simple colored square as icon
        if found_icon.is_none() {
            let size = 32;
            let mut rgba = Vec::with_capacity(size as usize * size as usize * 4);
            for _ in 0..size * size {
                rgba.extend_from_slice(&[255, 100, 100, 255]); // Red square
            }
            found_icon = Some(tray_icon::Icon::from_rgba(rgba, size, size).unwrap_or_else(|_| {
                // If even the fallback fails, create a minimal icon
                let mut minimal_rgba = Vec::with_capacity(16 * 16 * 4);
                for _ in 0..16 * 16 {
                    minimal_rgba.extend_from_slice(&[255, 100, 100, 255]);
                }
                tray_icon::Icon::from_rgba(minimal_rgba, 16, 16).unwrap()
            }));
        }
        
        found_icon.unwrap()
    };

    let attrs = TrayIconAttributes {
        icon: Some(icon),
        menu: Some(Box::new(tray_menu)),
        tooltip: Some("Konductor".into()),
        ..Default::default()
    };

    TrayIcon::new(attrs).expect("failed to create tray icon")
}

pub fn handle_system_tray_event(app: &AppHandle, event: TrayIconEvent) {
    match event {
        TrayIconEvent::Click { .. } => {
            if let Some(window) = app.get_webview_window("main") {
                if let Ok(is_visible) = window.is_visible() {
                    if is_visible {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        }
        _ => {}
    }
}

pub fn handle_menu_event(app: &AppHandle, event: MenuEvent) {
    if let Some(window) = app.get_webview_window("main") {
        match event.id.0.as_str() {
            "Show Konductor" => {
                let _ = window.show();
                let _ = window.set_focus();
            }
            "Hide Konductor" => {
                let _ = window.hide();
            }
            "Quit" => {
                std::process::exit(0);
            }
            _ => {}
        }
    }
}
