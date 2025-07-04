
# High-Performance Concurrent Web Crawler in Rust

![Rust](https://img.shields.io/badge/rust-1.78.0-orange.svg)
![Crates.io](https://img.shields.io/badge/crates-tokio,_reqwest,_clap-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

## What is a Web Crawler?

A web crawler is a program that starts from one webpage (called a seed URL) and visits all the links it finds on that page, then continues the process on each newly found page. It helps in collecting information, indexing websites, or analyzing website structures.

## Project Overview

This is a fast and concurrent web crawler written in Rust. It starts from a user-specified URL, visits pages up to a set depth, and extracts all valid hyperlinks. The crawler avoids revisiting the same URL, respects concurrency limits, and stores the page data efficiently.

## Core Rust Concepts Used

- **Asynchronous Programming (`async/await`, `tokio`)**: Handles multiple web requests concurrently without blocking threads.
- **Concurrency (`Arc`, `Mutex`, `Semaphore`)**: Shares data safely across tasks and limits the number of parallel requests.
- **Type Safety and Error Handling (`Result`, `thiserror`)**: Ensures reliable, predictable behavior even when requests fail.
- **Ownership and Borrowing**: Prevents memory bugs and ensures thread safety at compile time.
- **Modules and Crates**: Project is modular, with responsibilities divided into separate files.
- **Concurrent Task Lifecycle with `JoinSet`** — A robust architecture for managing the lifecycle of spawned crawl tasks (improved from a channel-based model for better task tracking and safety).

## Folder Structure

```
src/
│
├── main.rs         # CLI entry point and crawler initialization
├── crawler.rs      # Core logic for managing crawl workflow
├── fetcher.rs      # Handles HTTP requests with delay control
├── parser.rs       # Parses HTML and extracts links
├── storage.rs      # Tracks visited URLs and stores page data
```

## Workflow Diagram

![image](https://github.com/user-attachments/assets/2e0cb6a4-30da-492e-9010-72020d28137d)

## Code Workflow

1. **Input**: CLI arguments set start URL, depth, and concurrency.
2. **Initialize**: Build a shared `CrawlerConfig` and necessary components.
3. **JoinSet Spawning**: Start crawl tasks using `tokio::task::JoinSet`.
4. **Fetch**: Request HTML using `reqwest`.
5. **Parse**: Use `scraper` to extract valid hyperlinks.
6. **Track & Store**: Save page content and mark URLs as visited.
7. **Queue New Tasks**: If within depth limit, spawn new crawl tasks for discovered links.
8. **Complete**: Print a structured summary once the crawl finishes.

## Features

- ✅ Asynchronous Architecture using `tokio`
- ✅ Concurrency Task Management using `tokio::task::JoinSet`
- ✅ Rate Limiting to avoid flooding target servers using `Semaphore`
- ✅ Skips duplicate links with `DashSet`
- ✅ Clean logging with `tracing`
- ✅ CLI interface using `clap`
- ✅ Modular code structure

## Output

![image](https://github.com/user-attachments/assets/809018c6-04c3-4edc-ac84-56ec74e14c90)


## How to Use

### 1. Clone the Repository

```bash
git clone https://github.com/Rishi2333/rust-webcrawler.git
cd rust-webcrawler
```

### 2. Build the Project

```bash
cargo build --release
```

### 3. Run the Crawler

```bash
cargo run --release -- --url https://example.com --depth 2 --concurrency 20
```

### CLI Options

- `--url`: (Required) Starting point for the crawl
- `--depth`: (Optional) Max link depth (default: 2)
- `--max-pages`: (Optional) Max pages per domain (default: 50)
- `--concurrency`: (Optional) Parallel requests limit (default: 10)

### Example

```bash
cargo run --release -- --url https://quotes.toscrape.com --depth 1 --concurrency 10
```

## Future Enhancements

- Support for `robots.txt`
- Save results to SQLite/PostgreSQL
- Text content analysis
- Multi-domain crawl filtering

## License

This project is licensed under the MIT License.

