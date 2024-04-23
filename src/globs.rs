use glob;
use std::sync::Mutex;
use once_cell::sync::Lazy;

use crate::log;

static INCLUDE_GLOBS: Lazy<Mutex<Vec<glob::Pattern>>> = Lazy::new(|| Mutex::new(Vec::new()));
static EXCLUDE_GLOBS: Lazy<Mutex<Vec<glob::Pattern>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn set_includes(includes: Vec<String>) {
  let mut patterns = Vec::new();

  for include in includes {
    log!("Including: {}", include);
    let pattern = glob::Pattern::new(&include).expect(&format!("Invalid include glob pattern: {}", include));
    patterns.push(pattern);
  }

  let mut guard = INCLUDE_GLOBS.lock().unwrap();
  *guard = patterns;
}

pub fn set_excludes(excludes: Vec<String>) {
  let mut patterns = Vec::new();

  for exclude in excludes {
    log!("Excluding: {}", exclude);
    let pattern = glob::Pattern::new(&exclude).expect(&format!("Invalid exclude glob pattern: {}", exclude));
    patterns.push(pattern);
  }

  let mut guard = EXCLUDE_GLOBS.lock().unwrap();
  *guard = patterns;
}

pub fn path_is_valid(path: &str) -> bool {
  let includes = INCLUDE_GLOBS.lock().unwrap();
  let excludes = EXCLUDE_GLOBS.lock().unwrap();

  // First see if it's excluded...
  if !excludes.is_empty() {
    for exclude in &*excludes {
      if exclude.matches(path) {
        return false;
      }
    }
  }

  // Then see if it's included...
  if includes.is_empty() {
    return true;
  }

  for include in &*includes {
    log!("Checking if {} matches {}", path, include.as_str());
    if include.matches(path) {
      return true;
    }
  }

  false
}