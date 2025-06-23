// src/fetcher.rs

use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use url::Url;

use crate::crawler::{CrawlConfig, CrawlerError, CrawlerResult};

/// Responsible for fetching web content with rate-limiting per host.
pub struct Fetcher {
    client: Client,
    config: Arc<CrawlConfig>,
    last_access: Arc<Mutex<HashMap<String, Instant>>>, // Tracks last request time per host
}

impl Fetcher {
    /// Creates a new Fetcher instance with the given shared crawler configuration.
    pub fn new(config: Arc<CrawlConfig>) -> Self {
        let client = Client::builder()
            .user_agent(&config.user_agent)
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            config,
            last_access: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Fetches the content of a URL, applying delay if needed to avoid overloading a domain.
    pub async fn fetch(&self, url: &Url) -> CrawlerResult<String> {
        let host = url.host_str().ok_or_else(|| {
            CrawlerError::UrlParseError(url::ParseError::EmptyHost)
        })?;

        {
            let mut access_times = self.last_access.lock().await;
            let now = Instant::now();
            let min_delay = Duration::from_millis(self.config.delay_ms);

            // If the last request to this host was too recent, wait before sending the next one
            if let Some(last_time) = access_times.get(host) {
                let elapsed = now.duration_since(*last_time);
                if elapsed < min_delay {
                    let sleep_time = min_delay - elapsed;
                    tokio::time::sleep(sleep_time).await;
                }
            }

            // Update the last access time for this host
            access_times.insert(host.to_string(), Instant::now());
        }

        // Send HTTP GET request and check for HTTP errors
        let response = self.client.get(url.as_str()).send().await?;
        let checked = response.error_for_status()?;

        // Return the response body as text
        Ok(checked.text().await?)
    }
}
