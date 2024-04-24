use gumdrop::Options;
use mime_guess::from_path;
use std::{path::PathBuf, str::FromStr};
use tiny_http::{Header, Response, Server};

use crate::log::set_silent;

mod globs;
mod html;
mod log;
mod open;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const HASH: Option<&str> = option_env!("GIT_HASH");

#[derive(Debug, Options)]
struct Args {
  #[options(help = "Print help")]
  help: bool,

  #[options(help = "Print version")]
  version: bool,

  #[options(free)]
  path: Option<std::path::PathBuf>,

  #[options(help = "Disable logging to stdout")]
  quiet: bool,

  #[options(help = "Port to listen on", default = "8080")]
  port: u16,

  #[options(
    help = "Enable serving index.html or index.htm if path is /",
    default = "false"
  )]
  root_index: bool,

  #[options(
    help = "Serve an HTML document listing files and directories, when navigated to a directory",
    default = "false",
    short = "d"
  )]
  serve_directories: bool,

  #[options(help = "List of glob patterns to include", meta = "GLOB")]
  include: Vec<String>,

  #[options(help = "List of glob patterns to exclude", meta = "GLOB")]
  exclude: Vec<String>,

  #[options(help = "Open the browser after starting the server", default = "false")]
  open: bool,
}

pub fn main() {
  let opts = Args::parse_args_default_or_exit();
  let port = opts.port;
  let server = Server::http(format!("127.0.0.1:{}", port)).unwrap();
  let local_path = opts.path.unwrap_or(std::path::PathBuf::from("."));

  if opts.version {
    println!(
      "swerve {}-{}",
      VERSION.unwrap_or("unknown"),
      HASH.unwrap_or("unknown")
    );
    return;
  }

  set_silent(opts.quiet);

  // If the path is the current dir, warn just in case
  if local_path == PathBuf::from(".") {
    warn!("Serving current directory");
  }

  // Set includes and excludes
  globs::set_includes(opts.include);
  globs::set_excludes(opts.exclude);

  log!("Serving path: {:?}", local_path);
  log!(
    "Access by visiting http://127.0.0.1:{} in your browser",
    port
  );

  // Open in default browser
  if opts.open {
    open::open_in_browser(&format!("http://127.0.0.1:{}", port));
  }

  for request in server.incoming_requests() {
    let start = std::time::Instant::now();
    // Remove leading slash
    let path = request.url().strip_prefix('/').unwrap_or(request.url());
    let path = local_path.join(PathBuf::from(path));

    log!("Incoming request for {:?}", path);

    // If the path is nothing (root), look for index.html or index.htm
    let path = if opts.root_index && path == PathBuf::from("./") {
      log!("Looking for index.html or index.htm");
      let idx_files = ["index.html", "index.htm"];
      idx_files
        .iter()
        .map(|f| local_path.join(PathBuf::from(f)))
        .find(|f| f.exists())
        .unwrap_or(path)
    } else {
      path
    };

    // See if the path is valid
    if !globs::path_is_valid(path.to_str().unwrap()) {
      log!("Path is invalid due to glob patterns");
      request
        .respond(Response::empty(404))
        .expect("Failed to respond with 404");
      continue;
    }

    // If the path is a directory, serve the directory
    let response = if path.is_dir() && opts.serve_directories {
      let html = html::get_directory_html(&local_path, request.url());
      let mut res = Response::from_string(html);

      res.add_header(Header::from_str("Content-Type: text/html").unwrap());

      request.respond(res)
    } else {
      match std::fs::read(&path) {
        Ok(content) => {
          let mime = from_path(&path).first_or_text_plain();
          let mut res = Response::from_data(content.clone());

          // Headers
          let content_type = Header::from_str(format!("Content-Type: {}", mime).as_str()).unwrap();
          let content_length =
            Header::from_str(format!("Content-Length: {}", content.len()).as_str()).unwrap();

          res.add_header(content_type);
          res.add_header(content_length);

          request.respond(res)
        }
        Err(_) => request.respond(Response::empty(404)),
      }
    };

    // Suppress/handle error
    match response {
      Ok(_) => {
        success!("Reponse served for {:?}", path);
      }
      Err(e) => error!("Failed to serve {:?}: {:?}", path, e),
    }

    log!("Request took {:?}", start.elapsed());
  }
}
