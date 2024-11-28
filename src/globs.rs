use std::sync::OnceLock;

use crate::{error, log};

static INCLUDE_GLOBS: OnceLock<Vec<glob::Pattern>> = OnceLock::new();
static EXCLUDE_GLOBS: OnceLock<Vec<glob::Pattern>> = OnceLock::new();

pub fn set_includes(includes: Vec<String>) {
  let mut patterns = Vec::new();

  for include in includes {
    log!("Including: {}", include);
    let pattern = glob::Pattern::new(&include)
      .unwrap_or_else(|_| panic!("Invalid include glob pattern: {}", include));
    patterns.push(pattern);
  }

  INCLUDE_GLOBS
    .set(patterns)
    .unwrap_or_else(|e| error!("Failed to set include globs: {:?}", e));
}

pub fn set_excludes(excludes: Vec<String>) {
  let mut patterns = Vec::new();

  for exclude in excludes {
    log!("Excluding: {}", exclude);
    let pattern = glob::Pattern::new(&exclude)
      .unwrap_or_else(|_| panic!("Invalid exclude glob pattern: {}", exclude));
    patterns.push(pattern);
  }

  EXCLUDE_GLOBS
    .set(patterns)
    .unwrap_or_else(|e| error!("Failed to set exclude globs: {:?}", e));
}

pub fn path_is_valid(path: &str) -> bool {
  let includes = INCLUDE_GLOBS.get_or_init(Vec::new);
  let excludes = EXCLUDE_GLOBS.get_or_init(Vec::new);

  // First see if it's excluded...
  if !excludes.is_empty() {
    for exclude in excludes {
      if exclude.matches(path) {
        return false;
      }
    }
  }

  // Then see if it's included...
  if includes.is_empty() {
    return true;
  }

  for include in includes {
    log!("Checking if {} matches {}", path, include.as_str());
    if include.matches(path) {
      return true;
    }
  }

  false
}
