// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use auto_launch::AutoLaunchBuilder; // Import AutoLaunchBuilder explicitly.

mod indicator;
mod Configurations;

fn main() {
  let exe_path = std::env::current_exe();

  match exe_path {
    Ok(path) => {
      if let Some(installation_location) = path.parent() {
        let mut installation_location_str = installation_location.to_string_lossy().to_string();
        installation_location_str+="\\konductor";
        let auto = AutoLaunchBuilder::new()
          .set_app_name("konductor")
          .set_app_path(installation_location_str.as_str())
          .set_use_launch_agent(true)
          .build()
          .unwrap();

        auto.enable().unwrap();
        let is_enabled = auto.is_enabled().unwrap();
        if is_enabled {
          println!("Auto-launch is enabled. {:?}", installation_location_str);
        } else {
          println!("Auto-launch is not enabled.");
        }

        // auto.disable().unwrap();
        // let is_enabled = auto.is_enabled().unwrap();
        // if is_enabled {
        //   println!("Auto-launch is still enabled after disabling. You might want to handle this case.");
        // } else {
        //   println!("Auto-launch is disabled.");
        // }
      } else {
        eprintln!("Installation location is None.");
      }
    }
    Err(err) => {
      eprintln!("Error: {:?}", err);
    }
  }

  // The rest of your Tauri code seems fine, assuming you have the required event handlers in the indicator module.
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![indicator::get_cpu_usage, indicator::get_ram_usage, indicator::get_memory_usage])
    .run(tauri::generate_context!())
    .expect("error while running Tauri application");
}
