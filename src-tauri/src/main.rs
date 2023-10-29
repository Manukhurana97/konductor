// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;

mod indicator;
mod windowsOS;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![indicator::get_cpu_usage, indicator::get_ram_usage, indicator::get_memory_usage])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

