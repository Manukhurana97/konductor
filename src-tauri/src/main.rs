use sysinfo::System;
use sysinfo::Disks;

mod tray_linux;
use tray_linux::{create_system_tray, handle_menu_event, handle_system_tray_event};

#[tauri::command]
fn get_system_info() -> Result<serde_json::Value, String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // RAM in kB
    let total_ram = sys.total_memory();
    let used_ram = sys.used_memory();

    // CPU
    let total_cpu = sys.cpus().len();

    // Disk usage (sum of all disks)
    let disks = sysinfo::Disks::new_with_refreshed_list();
    let total_disk: u64 = disks.list().iter().map(|d| d.total_space()).sum();
    let used_disk: u64 = disks
        .list()
        .iter()
        .map(|d| d.total_space() - d.available_space())
        .sum();

    Ok(serde_json::json!({
        "total_ram": total_ram,
        "used_ram": used_ram,
        "total_cpu": total_cpu,
        "total_disk": total_disk,
        "used_disk": used_disk
    }))
}

fn main() {
    // Initialize GTK for tray icon support (Linux only)
    #[cfg(target_os = "linux")]
    {
        if let Err(e) = gtk::init() {
            eprintln!("Failed to initialize GTK: {}", e);
            // Continue without GTK if it fails
        }
    }
    
    let tray_icon = create_system_tray();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_system_info])
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Global channels provided by tray-icon
            let app_handle_tray = app_handle.clone();
            std::thread::spawn(move || {
                while let Ok(event) = tray_icon::TrayIconEvent::receiver().recv() {
                    handle_system_tray_event(&app_handle_tray, event);
                }
            });

            let app_handle_menu = app_handle.clone();
            std::thread::spawn(move || {
                while let Ok(event) = tray_icon::menu::MenuEvent::receiver().recv() {
                    handle_menu_event(&app_handle_menu, event);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
