use std::path::Path;

use crate::log;

static HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <body>
    <h1>Index of __DIRECTORY__</h1>

    <ul>
      <li><a href="..">..</a></li>
      __DIRS__
      __FILES__
    </ul>
  </body>

  <style>
    ul {
      list-style-type: none;
      margin: 0;
      padding: 0;

      border-top: 1px solid #000;
      border-bottom: 1px solid #000;
    }

    li {
      padding: 0.5em 0;
      border-bottom: 1px solid #777;
      vertical-align: middle;
    }

    li:last-child {
      border-bottom: none;
    }
</html>
"#;

pub fn get_directory_html(root: &Path, path: &str) -> String {
  let path = if path.starts_with('/') {
    ".".to_owned() + path
  } else {
    path.to_string()
  };
  let path = path.as_str();
  let mut dirs = String::new();
  let mut files = String::new();
  let full_path = root.join(path);
  let pretty_path = &full_path
    .to_string_lossy()
    .replace('\\', "/")
    .replace("./", "");
  let pretty_path = if pretty_path.is_empty() {
    "/".to_string()
  } else {
    pretty_path.to_string()
  };

  let dir = full_path.read_dir();

  // Sort by directory name
  let dir = match dir {
    Ok(dir) => dir,
    Err(_) => return "Failed to read directory".to_string(),
  };

  let mut dir: Vec<_> = dir.collect();
  dir.sort_by(|a, b| {
    let a = a.as_ref().unwrap().file_name();
    let b = b.as_ref().unwrap().file_name();
    a.cmp(&b)
  });

  for entry in dir {
    let entry = match entry {
      Ok(e) => e,
      Err(_) => {
        log!("Failed to read entry: {:?}", entry);
        continue;
      }
    };
    let name = entry.file_name().into_string().unwrap();
    let mut path_as_str = path.replace('\\', "/").replace("./", "/");

    if path_as_str == "/" {
      path_as_str = "".to_string();
    }

    if entry.file_type().unwrap().is_dir() {
      let href = format!("<a href=\"{}/{}\">{}/</a>", path_as_str, name, name);
      dirs.push_str(&format!("<li>{}</li>", href));
    } else {
      let href = format!("<a href=\"{}/{}\">{}</a>", path_as_str, name, name);
      files.push_str(&format!("<li>{}</li>", href));
    }
  }

  HTML
    .replace("__DIRECTORY__", &pretty_path)
    .replace("__DIRS__", &dirs)
    .replace("__FILES__", &files)
}
