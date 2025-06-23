// src/crawler.rs

use thiserror::Error;
use tokio::task::JoinSet;
use url::Url;

/// Defines all possible errors that can occur during crawling.
#[derive(Error, Debug)]
pub enum CrawlerError {
    #[error("Network error occurred: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Invalid URL provided: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("File system error: {0}")]
    IoError(#[from] std::io::Error),
}

/// A type alias for simplifying Result types in crawler logic.
pub type CrawlerResult<T> = Result<T, CrawlerError>;

/// Configuration options for controlling the crawling behavior.
#[derive(Clone, Debug)]
pub struct CrawlConfig {
    pub max_depth: usize,
    pub max_pages_per_domain: usize,
    pub user_agent: String,
    pub concurrent_requests: usize,
    pub delay_ms: u64,
}

impl Default for CrawlConfig {
    fn default() -> Self {
        Self {
            max_depth: 3,
            max_pages_per_domain: 100,
            user_agent: "Rust-Webcrawler/0.1".into(),
            concurrent_requests: 10,
            delay_ms: 100,
        }
    }
}

/// Represents a crawled page and its metadata.
#[derive(Debug, Clone)]
pub struct Page {
    pub url: Url,
    pub depth: usize,
    pub content: String,
    pub links: Vec<Url>,
}

use crate::fetcher::Fetcher;
use crate::parser::Parser;
use crate::storage::Storage;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing::{error, info, warn};

/// Main crawler structure that orchestrates fetching, parsing, and storing pages.
#[derive(Clone)]
pub struct Crawler {
    config: Arc<CrawlConfig>,
    fetcher: Arc<Fetcher>,
    parser: Arc<Parser>,
    storage: Arc<Storage>,
}

impl Crawler {
    /// Creates a new Crawler instance with the given configuration.
    pub fn new(config: CrawlConfig) -> Self {
        let config = Arc::new(config);
        let fetcher = Arc::new(Fetcher::new(config.clone()));
        let parser = Arc::new(Parser::new());
        let storage = Arc::new(Storage::new());

        Self {
            config,
            fetcher,
            parser,
            storage,
        }
    }

    /// Starts the crawling process from the given starting URL.
    pub async fn crawl(&self, start_url: &str) -> CrawlerResult<Vec<Page>> {
        let mut set = JoinSet::new();
        let semaphore = Arc::new(Semaphore::new(self.config.concurrent_requests));
        
        let start_url = Url::parse(start_url)?;

        // Avoid revisiting already processed URLs.
        if self.storage.mark_visited(&start_url) {
            set.spawn(self.clone().process_url(start_url, 0, semaphore.clone()));
        }

        // Process tasks as they complete.
        while let Some(res) = set.join_next().await {
            match res {
                Ok(Ok((new_links, depth))) => {
                    // Continue crawling deeper if depth limit hasn't been reached.
                    if depth < self.config.max_depth {
                        for link in new_links {
                            if self.storage.mark_visited(&link) {
                                set.spawn(self.clone().process_url(link, depth + 1, semaphore.clone()));
                            }
                        }
                    }
                },
                Ok(Err(e)) => {
                    error!("Worker task failed: {:?}", e);
                },
                Err(e) => {
                    error!("Failed to join task: {:?}", e);
                }
            }
        }

        // Return all successfully crawled pages.
        Ok(self.storage.get_pages().await)
    }

    /// Handles fetching, parsing, and storing a single URL.
    async fn process_url(self, url: Url, depth: usize, semaphore: Arc<Semaphore>) -> CrawlerResult<(Vec<Url>, usize)> {
        // Skip processing if domain page limit is exceeded.
        if let Some(host) = url.host_str() {
            if self.storage.domain_page_count(host).await >= self.config.max_pages_per_domain {
                warn!("Skipping {}: domain page limit reached", url);
                return Ok((vec![], depth));
            }
        }

        // Acquire a permit for concurrency control.
        let _permit = semaphore.acquire().await.expect("Semaphore closed unexpectedly");
        info!("Fetching (depth {}): {}", depth, url);

        // Fetch and parse page content.
        let content = self.fetcher.fetch(&url).await?;
        let page = self.parser.parse(&url, content, depth)?;

        let new_links = page.links.clone();
        self.storage.store_page(page).await;

        Ok((new_links, depth))
    }
}
