use std::process::Command;

#[cfg(target_os = "windows")]
pub fn open_in_browser(url: &str) {
  Command::new("cmd")
    .args(["/C", "start", url])
    .spawn()
    .expect("Failed to open browser");
}

#[cfg(target_os = "macos")]
pub fn open_in_browser(url: &str) {
  Command::new("open")
    .arg(url)
    .spawn()
    .expect("Failed to open browser");
}

#[cfg(target_os = "linux")]
pub fn open_in_browser(url: &str) {
  Command::new("xdg-open")
    .arg(url)
    .spawn()
    .expect("Failed to open browser");
}
