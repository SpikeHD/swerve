use reqwest::blocking::get;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::thread;

fn main() {
  let request_count = 1000;
  let mut serve_process = Command::new("cmd")
    .args(["/C", "npx", "serve"])
    .stdout(Stdio::null())
    .spawn()
    .expect("Failed to start npx serve");

  println!("Waiting for npx serve to start...");
  thread::sleep(Duration::from_secs(2));

  // Test
  println!("Testing npx serve...");
  let serve_duration = benchmark_request("http://127.0.0.1:3000/Cargo.toml", request_count);
  println!("npx serve response time ({} requests): {:?}", request_count, serve_duration);

  serve_process.kill().expect("Failed to stop npx serve");

  let mut swerve_process = Command::new("cmd")
    .args(["/C", "swerve", "--port", "8080"])
    .stdout(Stdio::null())
    .spawn()
    .expect("Failed to start swerve");

  println!("Waiting for swerve to start...");
  thread::sleep(Duration::from_secs(2));

  // Test
  println!("Testing swerve...");
  let swerve_duration = benchmark_request("http://127.0.0.1:8080/Cargo.toml", request_count);
  println!("swerve response time ({} requests): {:?}", request_count, swerve_duration);

  swerve_process.kill().expect("Failed to stop swerve");

  let serve_micros_flt = micros_to_f64(serve_duration.as_micros());
  let swerve_micros_flt = micros_to_f64(swerve_duration.as_micros());

  // Compare the results
  if serve_duration > swerve_duration {
    println!("npx serve is {:.2}x slower than swerve", serve_micros_flt / swerve_micros_flt);
  } else {
    println!("swerve is {:.2}x slower than npx serve", serve_micros_flt / swerve_micros_flt);
  }
}

fn micros_to_f64(micros: u128) -> f64 {
  match u32::try_from(micros) {
    Ok(micros) => f64::from(micros),
    Err(_) => panic!("Benchmark result too large"),
  }
}

fn benchmark_request(url: &str, n: u32) -> Duration {
  let mut times = Vec::new();
  let mut amount = n;

  for _ in 0..amount {
    let start = Instant::now();
    let _response = match get(url) {
      Ok(r) => r,
      Err(e) => {
        println!("Request failed ({:?}), retrying...", e);
        std::thread::sleep(Duration::from_millis(100));
        amount -= 1;
  
        continue;
      }
    };

    times.push(start.elapsed());
  }
  
  // Avg the results
  let sum: Duration = times.iter().sum();
  sum / n
}