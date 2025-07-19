use tauri::Manager;
use sysinfo::System;
use sysinfo::Disks;

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
    let disks = Disks::new_with_refreshed_list();
    let total_disk: u64 = disks.list().iter().map(|d| d.total_space()).sum();
    let used_disk: u64 = disks.list().iter().map(|d| d.total_space() - d.available_space()).sum();

    Ok(serde_json::json!({
        "total_ram": total_ram,
        "used_ram": used_ram,
        "total_cpu": total_cpu,
        "total_disk": total_disk,
        "used_disk": used_disk
    }))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_system_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
