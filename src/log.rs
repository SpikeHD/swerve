use chrono::Local;
use colored::Colorize;
use std::{fmt::Display, sync::atomic::AtomicBool};

static SILENT: AtomicBool = AtomicBool::new(false);

pub enum LogKind {
  Info,
  Success,
  Warn,
  Error,
}

pub fn set_silent(silent: bool) {
  SILENT.store(silent, std::sync::atomic::Ordering::Relaxed);
}

pub fn log(s: impl AsRef<str> + Display, kind: Option<LogKind>) {
  if SILENT.load(std::sync::atomic::Ordering::Relaxed) {
    return;
  }

  let status = match kind {
    Some(LogKind::Info) => "INFO".blue(),
    Some(LogKind::Success) => "DONE".green(),
    Some(LogKind::Warn) => "WARN".yellow(),
    Some(LogKind::Error) => "FAIL".red(),
    None => "INFO".blue(),
  };

  println!(
    "[{}] [{}] {}",
    Local::now().format("%Y-%m-%d %H:%M:%S"),
    status,
    s
  );
}

#[macro_export]
macro_rules! log {
  ($($arg:tt)*) => {
    $crate::log::log(format!($($arg)*), Some($crate::log::LogKind::Info))
  };
}

#[macro_export]
macro_rules! success {
  ($($arg:tt)*) => {
    $crate::log::log(format!($($arg)*), Some($crate::log::LogKind::Success))
  };
}

#[macro_export]
macro_rules! warn {
  ($($arg:tt)*) => {
    $crate::log::log(format!($($arg)*), Some($crate::log::LogKind::Warn))
  };
}

#[macro_export]
macro_rules! error {
  ($($arg:tt)*) => {
    $crate::log::log(format!($($arg)*), Some($crate::log::LogKind::Error))
  };
}
