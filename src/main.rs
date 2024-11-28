use colored::control;
use gumdrop::Options;
use mime_guess::from_path;
#[cfg(not(windows))]
use signal_hook::consts::{SIGINT, SIGTERM};
use std::{
  fs,
  net::{IpAddr, Ipv4Addr},
  path::PathBuf,
  str::FromStr,
  sync::Arc,
};
use threadpool::ThreadPool;
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
    help = "Serve index.html or index.htm if path is a directory containing such a file",
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

  #[options(
    help = "Bind to a specific address (or any, with 0.0.0.0)",
    default = "127.0.0.1",
    meta = "ADDRESS"
  )]
  bind: String,

  #[options(help = "Amount of threads to spawn for serving files", default = "1")]
  threads: usize,
}

pub fn main() {
  std::panic::set_hook(Box::new(|e| {
    error!("Panic: {:?}", e);
  }));

  #[cfg(target_os = "windows")]
  control::set_virtual_terminal(true).unwrap_or_default();

  let opts = Args::parse_args_default_or_exit();
  let port = opts.port;
  let server = Server::http(format!("{}:{}", opts.bind, port)).unwrap();
  // This is an Arc because it's used in the threadpool
  let local_path = Arc::new(opts.path.unwrap_or(std::path::PathBuf::from(".")));
  let addr = if opts.bind == "0.0.0.0" {
    local_ip_address::local_ip()
      .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
      .to_string()
  } else {
    opts.bind.clone()
  };

  if opts.version {
    println!(
      "swerve {}-{}",
      VERSION.unwrap_or("unknown"),
      HASH.unwrap_or("dev")
    );
    return;
  }

  set_silent(opts.quiet);

  // If the path is the current dir, warn just in case
  if local_path == Arc::new(PathBuf::from(".")) {
    warn!("Serving current directory");
  }

  // Set includes and excludes
  globs::set_includes(opts.include);
  globs::set_excludes(opts.exclude);

  if opts.threads == 1 {
    warn!(
      "Running in single-threaded mode! You may benefit from running with the --threads option"
    );
  }

  log!("Serving path: {:?}", local_path);
  log!(
    "Access by visiting http://{}:{} in your browser",
    addr,
    port
  );

  // Open in default browser
  if opts.open {
    open::open_in_browser(&format!("http://{}:{}", addr, port));
  }

  // Create signal handler
  #[cfg(not(windows))]
  std::thread::spawn(|| {
    let mut signals = signal_hook::iterator::Signals::new([SIGINT, SIGTERM])
      .expect("Failed to create signal iterator");

    for signal in signals.forever() {
      match signal {
        SIGINT | SIGTERM => {
          log!("Received signal {}, shutting down...", signal);
          std::process::exit(0);
        }
        _ => {}
      }
    }
  });

  let pool = ThreadPool::new(opts.threads);

  for request in server.incoming_requests() {
    let local_path = local_path.clone();

    pool.execute(move || {
      let start = std::time::Instant::now();
      // Remove leading slash
      let path = request.url().strip_prefix('/').unwrap_or(request.url());
      let mut path = local_path.join(PathBuf::from(path));

      log!("Incoming request for {:?}", path);

      // If the path is a dir but the URL does NOT end with a slash, redirect to version with slash
      if path.is_dir() && !request.url().ends_with('/') {
        warn!("URL does not have trailing slash, redirecting...");

        let mut res = Response::empty(301);
        res.add_header(Header::from_str(format!("Location: {}/", request.url()).as_str()).unwrap());
        request.respond(res).expect("Failed to respond with 301");
        return;
      }

      // If the path is nothing (root) or a directory, look for index.html or index.htm
      if opts.root_index {
        if let Ok(dir) = fs::read_dir(path.clone()) {
          log!("Looking for index.html or index.htm in {:?}", path);

          // Read the dir, look for index.html or index.htm
          let idx_files = ["index.html", "index.htm"];
          for entry in dir {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            if idx_files.contains(&entry_path.file_name().unwrap().to_str().unwrap()) {
              path = entry_path;
              break;
            }
          }
        }
      }

      // See if the path is valid
      if !globs::path_is_valid(path.to_str().unwrap()) {
        log!("Path is invalid due to glob patterns");
        request
          .respond(Response::empty(404))
          .expect("Failed to respond with 404");
        return;
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
            let content_type =
              Header::from_str(format!("Content-Type: {}", mime).as_str()).unwrap();
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
    });
  }
}
