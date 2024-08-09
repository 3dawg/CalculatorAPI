use reqwest::Client;
use std::time::Duration;
use tracing::{info, error};

pub async fn test_endpoints() {
    let client = Client::new();
    let base_url = "http://127.0.0.1:3000";

    let endpoints = vec![
        ("/", None),
        ("/add?a=5&b=3", Some("8")),
        ("/subtract?a=10&b=4", Some("6")),
        ("/multiply?a=7&b=2", Some("14")),
        ("/divide?a=20&b=4", Some("5")),
        ("/divide?a=20&b=0", Some("Error: Division by zero!")),
    ];

    for (endpoint, expected_response) in endpoints {
        let url = format!("{}{}", base_url, endpoint);
        match client.get(&url).timeout(Duration::from_secs(5)).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    let body = response.text().await.unwrap();
                    if let Some(expected) = expected_response {
                        if body.contains(expected) {
                            info!("Test passed for endpoint: {}", endpoint);
                        } else {
                            error!("Test failed for endpoint: {}. Expected: {} but got: {}", endpoint, expected, body);
                        }
                    } else {
                        info!("Test passed for endpoint: {}", endpoint);
                    }
                } else {
                    error!("Test failed for endpoint: {}. HTTP Status: {}", endpoint, response.status());
                }
            },
            Err(e) => error!("Test failed for endpoint: {}. Error: {}", endpoint, e),
        }
    }
}

pub fn init_logger() {
    tracing_subscriber::fmt::init();
    println!("Logger initialized!");
}
