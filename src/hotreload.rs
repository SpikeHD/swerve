use std::{net::TcpListener, path::PathBuf, sync::mpsc};
use notify::{recommended_watcher, Event, RecursiveMode, Result, Watcher};
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

pub fn watch(path: &PathBuf, event_tx: flume::Sender<Vec<PathBuf>>) -> Result<()> {
  let (tx, rx) = mpsc::channel::<Result<Event>>();
  let mut watcher = recommended_watcher(tx)?;

  watcher.watch(path, RecursiveMode::Recursive)?;

  loop {
    match rx.recv() {
      Ok(event) => {
        match event {
          Ok(event) => {
            if event.kind.is_modify() {
              log!("File modified: {:?}", event.paths);
              event_tx.send(event.paths).unwrap_or_else(|e| error!("Watcher send error: {:?}", e));
            }
          }
          Err(e) => error!("Watcher send error: {:?}", e),
        }
      },
      Err(e) => error!("Watcher send error: {:?}", e),
    };
  }
}

pub fn create_websocket(port: u16, event_rx: flume::Receiver<Vec<PathBuf>>) -> Result<()> {
  let rx = event_rx.clone();
  let server = TcpListener::bind(format!("127.0.0.1:{}", port))?;
  log!("Hotreload listening on port {}", port);
  
  for stream in server.incoming() {
    log!("Incoming hotreload connection");

    let stream = stream?;
    let rx = rx.clone();

    std::thread::spawn(move || {
      let rx = rx.clone();
      let mut uri = None;
      let mut headers = None;
      let websocket = accept_hdr(stream, |req: &tungstenite::http::Request<()>, res: tungstenite::http::Response<()>| {
        uri = Some(req.uri().to_string());
        headers = Some(res.headers().clone());
        Ok(res)
      });

      if let Ok(mut ws_stream) = websocket {
        ws_stream.get_mut().set_nonblocking(true).unwrap_or_default();

        loop {
          let msg = rx.try_recv();

          if msg.is_ok() {
            ws_stream.send(Message::Text("reload".to_string().into())).unwrap_or_else(|e| error!("Stream write error: {:?}", e));
          }

          // Also try to read from the stream
          if let Ok(msg) = ws_stream.read() {
            if msg.is_empty() || msg.is_close() {
              break;
            }
          }
        }
      }
    });
  }

  Ok(())
}