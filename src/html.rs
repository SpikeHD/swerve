use std::path::PathBuf;

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

pub fn get_directory_html(path: &PathBuf) -> String {
  let mut dirs = String::new();
  let mut files = String::new();

  for entry in path.read_dir().unwrap() {
    let entry = entry.unwrap();
    let name = entry.file_name().into_string().unwrap();
    let mut path_as_str = path.to_string_lossy().replace('\\', "/").replace("./", "/");

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

  let html = HTML
    .replace("__DIRECTORY__", &path.to_string_lossy().replace('\\', "/"))
    .replace("__DIRS__", &dirs)
    .replace("__FILES__", &files);

  html
}