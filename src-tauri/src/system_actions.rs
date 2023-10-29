use std::process::Command;

#[tauri::command]
pub fn shutdown() {
  match perform_shutdown() {
    Ok(_) => println!("System is shutting down."),
    Err(error) => eprintln!("Failed to shut down: {}", error),
  }
}

#[tauri::command]
pub fn restart() {
  match perform_restart() {
    Ok(_) => println!("System is restarting."),
    Err(error) => eprintln!("Failed to restart: {}", error),
  }
}


fn perform_shutdown() -> Result<(), std::io::Error> {
  // Implement your shutdown logic here, for example:
  let output = Command::new("shutdown").arg("-h").arg("now").output()?;
  if output.status.success() {
    Ok(())
  } else {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to shut down"))
  }
}

fn perform_restart() -> Result<(), std::io::Error> {
  // Implement your restart logic here, for example:
  let output = Command::new("reboot").output()?;
  if output.status.success() {
    Ok(())
  } else {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to restart"))
  }
}


