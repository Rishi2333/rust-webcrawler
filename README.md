# High-Performance Concurrent Web Crawler in Rust

![Rust](https://img.shields.io/badge/rust-1.78.0-orange.svg)
![Crates.io](https://img.shields.io/badge/crates-tokio,_reqwest,_clap-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

This project is a high-performance, asynchronous web crawler built from scratch in Rust. It was developed as a deep-dive into Rust's core strengths: memory safety, fearless concurrency, and zero-cost abstractions. The application can crawl websites starting from a given URL, respecting specified depth and concurrency limits.

## Features

* **Asynchronous:** Built on the `tokio` runtime for non-blocking I/O, allowing for thousands of in-flight requests.
* **Concurrent:** Safely processes multiple URLs simultaneously using lightweight asynchronous tasks.
* **Controlled Parallelism:** Uses a `Semaphore` to limit the number of concurrent network requests, preventing server overload.
* **Robust State Management:** Employs `DashSet` for high-throughput, thread-safe tracking of visited URLs to prevent loops.
* **Type-Safe Error Handling:** Uses a custom error `enum` with `thiserror` for clean and expressive error propagation.
* **User-Friendly CLI:** A command-line interface built with `clap` for easy configuration of crawl parameters.

## How to Run

1.  **Clone the repository:**
    ```bash
    git clone <your-repo-url>
    cd webcrawler
    ```

2.  **Run the crawler:**
    The command requires a starting URL and accepts optional arguments for depth and concurrency.
    ```bash
    cargo run --release -- --url [https://toscrape.com](https://toscrape.com) --depth 2 --concurrency 50
    ```
    * `--release`: Compiles the project with optimizations for maximum performance.
    * `--url`: The entry point for the crawl.
    * `--depth`: (Optional, default: 2) How many links deep to follow.
    * `--concurrency`: (Optional, default: 10) How many requests to run in parallel.

## Key Learnings

This project was a practical masterclass in several advanced programming concepts:
* **Fearless Concurrency:** Gained hands-on experience with `Arc<Mutex<T>>`, `mpsc::channel`, and `Semaphore` to write safe, multi-tasked code where the compiler guarantees against data races.
* **Asynchronous Rust:** Deep understanding of the `tokio` runtime, `async/await` syntax, and spawning non-blocking tasks.
* **Robust Error Handling:** Mastered the `Result<T, E>` and `?` operator pattern, a cornerstone of reliable Rust applications.
* **Leveraging the Ecosystem:** Learned to effectively integrate powerful third-party crates to accelerate development.
* **Debugging & OS Interaction:** Gained practical experience debugging not just code logic, but also runtime issues like OS-level file locks (`Access is denied`).

---

<details>
<summary><strong>Click to View Technical Deep-Dive & Architecture</strong></summary>

### Technical Deep-Dive: Implementing a Concurrent Web Crawler in Rust

#### **Abstract**

This document outlines the design and implementation of a high-performance, asynchronous web crawler in Rust...

***(Copy and paste the entire technical article I wrote for you in the previous response here)***

...The project successfully implements a robust, concurrent web crawler and demonstrates several of Rust's powerful features.

</details>

---

## Future Work

The project is ripe for improvements, including:
* **Respecting `robots.txt`:** Adding a module to parse and follow the rules set by website owners.
* **Persistent Storage:** Saving the crawled data to a database like SQLite or PostgreSQL.
* **Content Analysis:** Going beyond just fetching links to actually analyzing the text content of the pages.
