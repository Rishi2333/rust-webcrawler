use scraper::{Html, Selector};
use url::Url;
use tracing::debug;

use crate::crawler::{CrawlerResult, Page};

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    /// Extracts all valid HTTP/HTTPS links from the given HTML content.
    pub fn parse(&self, url: &Url, content: String, depth: usize) -> CrawlerResult<Page> {
        let document = Html::parse_document(&content);
        let link_selector = Selector::parse("a[href]").unwrap();
        let mut links = Vec::new();

        for element in document.select(&link_selector) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(absolute_url) = url.join(href) {
                    if absolute_url.scheme() == "http" || absolute_url.scheme() == "https" {
                        links.push(absolute_url);
                    }
                }
            }
        }

        debug!("Found {} links on page: {}", links.len(), url);

        Ok(Page {
            url: url.clone(),
            depth,
            content,
            links,
        })
    }
}
