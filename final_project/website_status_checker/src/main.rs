use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use reqwest::blocking::Client;

#[derive(Debug, Clone)]
struct WebsiteStatus {
    url: String,
    action_status: Result<u16, String>,
    response_time: Duration,
    timestamp: SystemTime,
}

struct Config {
    urls: Vec<String>,
    workers: usize,
    timeout: u64,
    retries: usize,
}

fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();
    let mut urls = Vec::new();
    let mut workers = num_cpus::get();
    let mut timeout = 5;
    let mut retries = 0;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing file path after --file".to_string());
                }
                let file = File::open(&args[i]).map_err(|e| e.to_string())?;
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    let line = line.map_err(|e| e.to_string())?;
                    let line = line.trim();
                    if !line.is_empty() && !line.starts_with('#') {
                        urls.push(line.to_string());
                    }
                }
            }
            "--workers" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing value after --workers".to_string());
                }
                workers = args[i].parse().map_err(|_| "Invalid workers value".to_string())?;
            }
            "--timeout" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing value after --timeout".to_string());
                }
                timeout = args[i].parse().map_err(|_| "Invalid timeout value".to_string())?;
            }
            "--retries" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing value after --retries".to_string());
                }
                retries = args[i].parse().map_err(|_| "Invalid retries value".to_string())?;
            }
            _ => {
                urls.push(args[i].clone());
            }
        }
        i += 1;
    }

    if urls.is_empty() {
        return Err("No URLs provided. Usage: website_checker [--file sites.txt] [URL ...] [--workers N] [--timeout S] [--retries N]".to_string());
    }

    Ok(Config {
        urls,
        workers,
        timeout,
        retries,
    })
}

fn check_website(url: &str, client: &Client, retries: usize) -> WebsiteStatus {
    let start = Instant::now();
    let mut attempt = 0;
    let mut result = Err("Initial failure".to_string());

    while attempt <= retries {
        match client.get(url).send() {
            Ok(resp) => {
                result = Ok(resp.status().as_u16());
                break;
            }
            Err(e) => {
                result = Err(e.to_string());
                attempt += 1;
                if attempt <= retries {
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }
    }

    WebsiteStatus {
        url: url.to_string(),
        action_status: result,
        response_time: start.elapsed(),
        timestamp: SystemTime::now(),
    }
}

fn generate_json(statuses: &[WebsiteStatus]) -> String {
    let mut json = String::from("[\n");
    for (i, status) in statuses.iter().enumerate() {
        let status_str = match &status.action_status {
            Ok(code) => code.to_string(),
            Err(e) => format!("\"{}\"", e.replace("\"", "\\\"")),
        };
        let timestamp = status
            .timestamp
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        json.push_str(&format!(
            "  {{\n    \"url\": \"{}\",\n    \"status\": {},\n    \"response_time_ms\": {},\n    \"timestamp\": {}\n  }}",
            status.url.replace("\"", "\\\""),
            status_str,
            status.response_time.as_millis(),
            timestamp
        ));
        if i < statuses.len() - 1 {
            json.push_str(",");
        }
        json.push_str("\n");
    }
    json.push_str("]");
    json
}

fn main() -> io::Result<()> {
    let config = match parse_args() {
        Ok(config) => Arc::new(config),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(2);
        }
    };

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(config.timeout))
        .build()
        .expect("Failed to build HTTP client");
    let client = Arc::new(client);

    let (tx, rx): (Sender<WebsiteStatus>, Receiver<WebsiteStatus>) = mpsc::channel();
    let rx = Arc::new(std::sync::Mutex::new(rx));

    let mut handles = Vec::new();
    for _ in 0..config.workers {
        let rx = Arc::clone(&rx);
        let config = Arc::clone(&config);
        let client = Arc::clone(&client);
        let handle = thread::spawn(move || {
            while let Ok(status) = rx.lock().unwrap().recv() {
                println!(
                    "{}: {} ({:?})",
                    status.url,
                    match &status.action_status {
                        Ok(code) => code.to_string(),
                        Err(e) => e.to_string(),
                    },
                    status.response_time
                );
            }
        });
        handles.push(handle);
    }

    let mut statuses = Vec::new();
    for url in &config.urls {
        let status = check_website(url, &client, config.retries);
        statuses.push(status.clone());
        tx.send(status).unwrap();
    }

    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }

    let json = generate_json(&statuses);
    std::fs::write("status.json", json)?;

    Ok(())
}