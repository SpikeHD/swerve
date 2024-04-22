use tide;
use async_std::task::block_on;

pub fn main() {
  let mut app = tide::new();
  let path = if let Some(arg) = std::env::args().nth(1) {
    std::path::PathBuf::from(arg)
  } else {
    std::env::current_dir().expect("Could not get current directory")
  };

  // Serve anything in the current directory
  app.at("/").serve_dir(&path).expect("Could not serve directory");

  println!("Serving {:?} on http://127.0.0.1:8080", path);

  block_on(app.listen("127.0.0.1:8080")).expect("Could not start server");
}