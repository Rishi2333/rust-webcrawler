// Declare internal modules
mod crawler;
mod fetcher;
mod parser;
mod storage;

// Import required libraries and project types
use clap::Parser as ClapParser;
use crawler::{CrawlConfig, Crawler};
use std::time::Instant;
use tracing::{info, Level};
use tracing_subscriber::fmt;

/// Defines CLI arguments accepted by the crawler application
#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The starting URL to begin crawling from
    #[arg(short, long)]
    url: String,

    /// Maximum depth to crawl (how many links deep)
    #[arg(short, long, default_value_t = 2)]
    depth: usize,

    /// Maximum number of pages to fetch per domain
    #[arg(long, default_value_t = 50)]
    max_pages: usize,

    /// Number of concurrent requests to run at a time
    #[arg(short, long, default_value_t = 10)]
    concurrency: usize,
}

/// Entry point of the application
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up structured logging output
    fmt::fmt()
        .with_max_level(Level::INFO)
        .without_time()
        .with_target(false)
        .init();

    let args = Args::parse();

    // Create crawler configuration using CLI arguments
    let config = CrawlConfig {
        max_depth: args.depth,
        max_pages_per_domain: args.max_pages,
        concurrent_requests: args.concurrency,
        ..Default::default()
    };

    let crawler = Crawler::new(config);
    let start_time = Instant::now();

    info!("Crawling started: URL = {}, Depth = {}", args.url, args.depth);

    // Begin the crawling process
    let pages = crawler.crawl(&args.url).await?;

    let elapsed = start_time.elapsed();
    info!(
        "Crawling completed! {} pages fetched in {:.2?} seconds.",
        pages.len(),
        elapsed
    );

    // Output a summary of up to 5 pages
    if !pages.is_empty() {
        info!("--- Crawl Report (Top 5 pages) ---");
        for page in pages.iter().take(5) {
            info!(
                "- URL: {} | Depth: {} | Content Size: {} bytes",
                page.url,
                page.depth,
                page.content.len()
            );
        }
        if pages.len() > 5 {
            info!("...and {} more pages were found.", pages.len() - 5);
        }
        info!("--- End of Report ---");
    }

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

//--RUN COMMANDS--//


//cargo run -- --url https://toscrape.com --depth 1

//cargo run --release -- --url http://books.toscrape.com/ --concurrency 50

//cargo run --release -- --url http://quotes.toscrape.com/ --depth 4 --max-pages 25 --output quotes_urls.txt


////cargo run -- --url https://www.indiatoday.in/ --depth 1