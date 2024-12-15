use notify::{recommended_watcher, Event, EventKind, RecursiveMode, Watcher};
use std::{
  net::{TcpListener, TcpStream},
  path::{Path, PathBuf},
  sync::mpsc,
};
use tungstenite::{accept_hdr, Message};

use crate::{error, log};

static HOTRELOAD_JS: &str = r#"
const ws = new WebSocket("ws://127.0.0.1:__PORT__")
ws.onmessage = function(event) {
  if (event.data === "reload") {
    location.reload()
  }
};

ws.onopen = (event) => {
  console.log("Hot-reload connected")
}

// On unload send disconnect message
window.beforeunload = () => {
  ws.close()
}
"#;

pub fn get_hotreload_js(port: u16) -> String {
  HOTRELOAD_JS.replace("__PORT__", &port.to_string())
}

pub fn watch(path: &Path, event_tx: flume::Sender<Vec<PathBuf>>) -> notify::Result<()> {
  let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
  let mut watcher = recommended_watcher(tx)?;

  watcher.watch(path, RecursiveMode::Recursive)?;

  loop {
    match rx.recv() {
      Ok(Ok(event)) => match event.kind {
        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
          log!("File changed ({:?}): {:?}", event.kind, event.paths);
          if let Err(e) = event_tx.send(event.paths) {
            error!("Watcher send error: {:?}", e);
          }
        }
        _ => {}
      },
      Ok(Err(e)) => error!("Watcher send error: {:?}", e),
      Err(e) => error!("Watcher receive error: {:?}", e),
    }
  }
}

pub fn create_websocket(
  port: u16,
  event_rx: flume::Receiver<Vec<PathBuf>>,
) -> Result<(), std::io::Error> {
  let rx = event_rx.clone();
  let server = TcpListener::bind(format!("127.0.0.1:{}", port))?;
  log!("Hotreload listening on port {}", port);

  for stream in server.incoming() {
    log!("Incoming hotreload connection");

    let stream = stream?;
    let rx = rx.clone();

    std::thread::spawn(move || ws_stream(rx, stream));
  }

  Ok(())
}

fn ws_stream(
  rx: flume::Receiver<Vec<PathBuf>>,
  stream: TcpStream,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  // TODO this may be able to be used for more advanced/optimized hotreload somehow
  let mut uri = None;
  let mut headers = None;
  let mut websocket = accept_hdr(
    stream,
    move |req: &tungstenite::http::Request<()>, res: tungstenite::http::Response<()>| {
      uri.replace(req.uri().to_string());
      headers.replace(res.headers().clone());
      Ok(res)
    },
  )?;

  websocket
    .get_mut()
    .set_nonblocking(true)
    .unwrap_or_default();

  loop {
    let msg = rx.try_recv();

    if msg.is_ok() {
      websocket
        .send(Message::Text("reload".to_string().into()))
        .unwrap_or_else(|e| error!("Stream write error: {:?}", e));
    }

    // Also try to read from the stream
    if let Ok(msg) = websocket.read() {
      if msg.is_empty() || msg.is_close() {
        break;
      }
    }
  }

  Ok(())
}
