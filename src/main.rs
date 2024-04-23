use gumdrop::Options;
use mime_guess::from_path;
use std::{path::PathBuf, str::FromStr};
use tiny_http::{Header, Response, Server};

use crate::log::set_silent;

mod log;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[derive(Debug, Options)]
struct Args {
  #[options(help = "Print help")]
  help: bool,

  #[options(free)]
  path: Option<std::path::PathBuf>,

  #[options(help = "Disable logging to stdout")]
  quiet: bool,

  #[options(help = "Port to listen on", default = "8080")]
  port: u16,

  #[options(help = "Print version")]
  version: bool,
}

pub fn main() {
  let opts = Args::parse_args_default_or_exit();
  let port = opts.port;
  let server = Server::http(format!("127.0.0.1:{}", port)).unwrap();
  let local_path = opts.path.unwrap_or(std::path::PathBuf::from("."));

  if opts.version {
    println!("swerve {}", VERSION.unwrap_or("unknown"));
    return;
  }

  set_silent(opts.quiet);

  // If the path is the current dir, warn just in case
  if local_path == PathBuf::from(".") {
    warn!("Serving current directory");
  }

  log!("Listening on port {}", port);
  log!("Serving path: {:?}", local_path);

  for request in server.incoming_requests() {
    let start = std::time::Instant::now();
    // Remove leading slash
    let path = request.url().strip_prefix('/').unwrap_or(request.url());
    let path = local_path.join(PathBuf::from(path));

    log!("Incoming request for {:?}", path);

    // If the path is nothing (root), look for index.html or index.htm
    let path = if path == PathBuf::from("./") {
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

    let response = match std::fs::read(&path) {
      Ok(content) => {
        let mime = from_path(&path).first_or_octet_stream();
        let mut response = Response::from_data(content.clone());

        // Headers
        let content_type = Header::from_str(format!("Content-Type: {}", mime).as_str()).unwrap();
        let content_length =
          Header::from_str(format!("Content-Length: {}", content.len()).as_str()).unwrap();

        response.add_header(content_type);
        response.add_header(content_length);

        request.respond(response)
      }
      Err(_) => request.respond(Response::empty(404)),
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
