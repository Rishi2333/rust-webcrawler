Coding partner

Show thinking
Excellent. This is a very clean, scannable, and technical style. Here is a README for your project written in this format.

High-Performance Concurrent Web Crawler in Rust
This project is a high-performance, asynchronous web crawler implemented in Rust. It is designed to crawl websites starting from a seed URL, extracting links and visiting them recursively within user-defined depth and concurrency limits. It demonstrates safe concurrency, efficient networking, and robust state management in Rust.

What Is a Web Crawler?
A web crawler (also known as a spider) is a program that systematically browses the World Wide Web. It starts from one or more initial URLs (seeds), downloads the page content, extracts all valid hyperlinks from the page, and adds unvisited links to a queue for future crawling. This process repeats recursively to traverse a portion of the web.

Features
Asynchronous Execution: Uses the tokio runtime to enable non-blocking I/O operations for high throughput.
Concurrent Task Management: Executes multiple crawling tasks simultaneously using tokio::task::JoinSet.
Controlled Parallelism: A Semaphore ensures a safe cap on the number of concurrent HTTP requests.
Link Extraction: Parses and resolves all valid <a href="..."> links using the scraper crate.
Thread-Safe State Tracking: Prevents duplicate visits using DashSet and tracks per-domain page counts.
Custom Error Types: Defines clean error types using thiserror for readable and maintainable error handling.
Configurable CLI: Users can specify the URL, depth, concurrency, and per-domain page limits via clap.
Usage
Clone the repository
Bash

git clone https://github.com/rishi2333/rust-webcrawler.git
cd rust-webcrawler
Build the project
Bash

cargo build --release
Run the crawler
Bash

cargo run --release -- --url https://example.com --depth 2 --concurrency 20
CLI Options
Flag	Description	Default
--url <URL>	Starting point of the crawl (required)	—
--depth <NUMBER>	Max recursion depth for following links	2
--max-pages <N>	Max pages to crawl per domain	50
--concurrency <N>	Max number of in-flight requests	10

Export to Sheets
Internal Architecture
&lt;details>
&lt;summary>&lt;strong>View Technical Breakdown&lt;/strong>&lt;/summary>

Modules
main.rs — Handles CLI parsing (clap) and initializes the crawl process.
crawler.rs — The core orchestrator. Manages the JoinSet, task spawning, depth checks, and worker coordination.
fetcher.rs — Handles timed and throttled HTTP GET requests using reqwest.
parser.rs — Extracts and resolves relative and absolute links from HTML content using scraper.
storage.rs — Tracks visited URLs (DashSet) and stores crawled page data (Arc<Mutex<Vec<Page>>>).
Workflow
CLI arguments are parsed into a CrawlConfig struct.
The seed URL is added as the first task in a JoinSet.
The main loop waits for tasks to complete.
Each completed worker task returns a list of new links it found.
New, unvisited links are spawned as new tasks in the JoinSet, respecting depth and domain limits.
The main thread waits for the JoinSet to become empty.
A summary report is printed to the console.
&lt;/details>

Key Concepts Practiced
Arc for safe, multi-threaded sharing of state (config, clients).
DashSet and Mutex for concurrent, mutable state management.
async/.await and the tokio runtime for non-blocking task execution.
Result<T, E> and the ? operator for clean and robust error propagation.
Structured logging and diagnostics via the tracing crate.
Future Enhancements
Add robots.txt support for crawl politeness rules.
Store results in a persistent database (e.g., SQLite).
Export crawled data as JSON or CSV.
Analyze text content for keyword extraction.
