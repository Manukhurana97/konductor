use crate::system_configurations;

#[tauri::command]
pub fn get_cpu_usage() -> i32 {
    return system_configurations::get_cpu_usage_windows().into();
}

#[tauri::command]
pub fn get_ram_usage() -> i32 {
    return system_configurations::get_ram_usage_windows().into();
}

#[tauri::command]
pub fn get_memory_usage() -> i32 {
    return system_configurations::get_memory_usage_window().into();
}

