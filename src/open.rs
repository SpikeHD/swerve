use std::process::Command;

pub fn open_in_browser(url: &str) {
  #[cfg(target_os = "windows")]
  {
    Command::new("cmd")
      .args(["/C", "start", url])
      .spawn()
      .expect("Failed to open browser");
  }

  #[cfg(target_os = "macos")]
  {
    Command::new("open")
      .arg(url)
      .spawn()
      .expect("Failed to open browser");
  }

  #[cfg(target_os = "linux")]
  {
    Command::new("xdg-open")
      .arg(url)
      .spawn()
      .expect("Failed to open browser");
  }
}
