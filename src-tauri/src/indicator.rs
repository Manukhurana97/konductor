use crate::windowsOS;

#[tauri::command]
pub fn get_cpu_usage() -> i32 {
    return windowsOS::get_cpu_usage_windows().into();
}

#[tauri::command]
pub fn get_ram_usage() -> i32 {
    return windowsOS::get_ram_usage_windows().into();
}

#[tauri::command]
pub fn get_memory_usage() -> i32 {
    return windowsOS::get_memory_usage_window().into();
}

