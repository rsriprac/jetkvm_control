#[cfg(target_os = "windows")]
pub mod windows_util;

#[cfg(target_os = "macos")]
pub mod macos_util;

use futures::stream::{FuturesUnordered, StreamExt};
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

pub async fn scan_chrome_debug_ports(start: u16, end: u16) -> Vec<u16> {
    let client = Client::builder()
        // make sure we don’t hang forever
        .timeout(Duration::from_millis(200))
        .build()
        .expect("building reqwest client");
    
    let mut futures = FuturesUnordered::new();
    for port in start..=end {
        let client = client.clone();
        futures.push(async move {
            let url = format!("http://127.0.0.1:{}/json/version", port);
            match client.get(&url).send().await {
                Ok(resp) if resp.status().is_success() => {
                    if let Ok(json) = resp.json::<Value>().await {
                        println!("Found Chrome at port {}", json);
                        // Chrome’s JSON will have a "Browser" field
                        if json.get("Browser").is_some() {
                            return Some(port);
                        }
                    }
                    None
                }
                _ => None,
            }
        });
    }

    let mut found = vec![];
    while let Some(opt) = futures.next().await {
        if let Some(port) = opt {
            found.push(port);
        }
    }
    found.sort_unstable();
    found
}