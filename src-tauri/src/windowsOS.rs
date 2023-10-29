extern crate sysinfo;

use sysinfo::{System, SystemExt, CpuExt, ProcessExt, DiskExt};



#[tauri::command]
pub fn get_cpu_usage_windows() -> i32 {
    let mut sys = System::new_all();
    sys.refresh_cpu();

    let mut usage = 0.0;
    for cpu in sys.cpus() {
        usage += cpu.cpu_usage();
    }

    return (usage / sys.cpus().len() as f32) as i32;
}

#[tauri::command]
pub fn get_ram_usage_windows() -> i32 {
    let mut sys = System::new_all();
    sys.refresh_memory();

    let memory = sys.total_memory();
    let used_memory = sys.used_memory();

    return (used_memory as f64 / memory as f64 * 100.0) as i32;
}

#[tauri::command]
pub fn get_memory_usage_window() -> i32 {
    let mut sys = System::new_all();
    sys.refresh_disks();

    let mut total_written_bytes = 0;
    let mut written_bytes = 0;
    let mut total_read_bytes = 0;
    let mut read_bytes = 0;
    let memory_size = sys.disks().iter().map(|disk| disk.total_space()).sum::<u64>() as f64;


    for (pid, process) in sys.processes() {
        total_written_bytes += process.disk_usage().total_written_bytes;
        written_bytes += process.disk_usage().written_bytes;
        total_read_bytes += process.disk_usage().total_read_bytes;
        read_bytes += process.disk_usage().read_bytes;
    }


    let total_bytes = (total_written_bytes + written_bytes + total_read_bytes + read_bytes) as f64;
    let percentage = ((total_bytes / memory_size ) * 100.0).round() as i32;
    return percentage;

    return -1;
}

