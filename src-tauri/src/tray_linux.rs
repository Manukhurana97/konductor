use tauri::{SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent, CustomMenuItem, Manager, AppHandle, Wry};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sysinfo::{System, SystemExt, CpuExt, ProcessExt, DiskExt};

// Shared state for stats
struct TrayStats {
    cpu: i32,
    ram: i32,
    disk: i32,
}

type SharedStats = Arc<Mutex<TrayStats>>;

fn get_stats() -> TrayStats {
    let mut sys = System::new_all();
    sys.refresh_cpu();
    sys.refresh_memory();
    sys.refresh_disks();

    // CPU usage (average across all cores)
    let mut cpu_usage = 0.0;
    for cpu in sys.cpus() {
        cpu_usage += cpu.cpu_usage();
    }
    let cpu = (cpu_usage / sys.cpus().len() as f32) as i32;

    // RAM usage (percentage)
    let memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let ram = if memory > 0 {
        (used_memory as f64 / memory as f64 * 100.0) as i32
    } else {
        0
    };

    // Disk usage (approximate, similar to your Windows logic)
    let mut total_written_bytes = 0;
    let mut written_bytes = 0;
    let mut total_read_bytes = 0;
    let mut read_bytes = 0;
    let memory_size = sys.disks().iter().map(|disk| disk.total_space()).sum::<u64>() as f64;
    for (_pid, process) in sys.processes() {
        total_written_bytes += process.disk_usage().total_written_bytes;
        written_bytes += process.disk_usage().written_bytes;
        total_read_bytes += process.disk_usage().total_read_bytes;
        read_bytes += process.disk_usage().read_bytes;
    }
    let total_bytes = (total_written_bytes + written_bytes + total_read_bytes + read_bytes) as f64;
    let disk = if memory_size > 0.0 {
        ((total_bytes / memory_size) * 100.0).round() as i32
    } else {
        0
    };

    TrayStats { cpu, ram, disk }
}

pub fn build_linux_tray() -> (SystemTray, SharedStats) {
    let stats = Arc::new(Mutex::new(get_stats()));
    let cpu_item = CustomMenuItem::new("cpu_stat", format!("CPU: {}%", stats.lock().unwrap().cpu)).disabled();
    let ram_item = CustomMenuItem::new("ram_stat", format!("RAM: {}%", stats.lock().unwrap().ram)).disabled();
    let disk_item = CustomMenuItem::new("disk_stat", format!("Disk: {}%", stats.lock().unwrap().disk)).disabled();
    let refresh = CustomMenuItem::new("refresh", "Refresh Stats");
    let show = CustomMenuItem::new("show", "Show App");
    let quit = CustomMenuItem::new("quit", "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(cpu_item)
        .add_item(ram_item)
        .add_item(disk_item)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(refresh)
        .add_item(show)
        .add_item(quit);
    (SystemTray::new().with_menu(tray_menu), stats)
}

pub fn start_stats_updater(app: AppHandle<Wry>, stats: SharedStats) {
    thread::spawn(move || {
        loop {
            let new_stats = get_stats();
            {
                let mut s = stats.lock().unwrap();
                s.cpu = new_stats.cpu;
                s.ram = new_stats.ram;
                s.disk = new_stats.disk;
            }
            // Update tray menu items
            let _ = app.tray_handle().set_menu(SystemTrayMenu::new()
                .add_item(CustomMenuItem::new("cpu_stat", format!("CPU: {}%", new_stats.cpu)).disabled())
                .add_item(CustomMenuItem::new("ram_stat", format!("RAM: {}%", new_stats.ram)).disabled())
                .add_item(CustomMenuItem::new("disk_stat", format!("Disk: {}%", new_stats.disk)).disabled())
                .add_native_item(SystemTrayMenuItem::Separator)
                .add_item(CustomMenuItem::new("refresh", "Refresh Stats"))
                .add_item(CustomMenuItem::new("show", "Show App"))
                .add_item(CustomMenuItem::new("quit", "Quit"))
            );
            thread::sleep(Duration::from_secs(5));
        }
    });
}

pub fn handle_tray_event(app: &AppHandle<Wry>, event: &SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "show" => {
                    if let Some(window) = app.get_window("main") {
                        let _ = window.show();
                    }
                }
                "refresh" => {
                    // Optionally trigger immediate stats update
                }
                _ => {}
            }
        }
        _ => {}
    }
} 