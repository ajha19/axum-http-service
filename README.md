# Axum API Gateway

A production oriented HTTP service built from scratch using Rust and Axum.

This project is designed to demonstrate how to build a real world async backend service with clean architecture, custom middleware, and correct lifecycle handling. The focus is on correctness, performance, and clarity rather than demo level shortcuts.

## Project Goals

- Build an HTTP service from zero using Axum and Tokio
- Implement custom middleware instead of relying on external crates
- Demonstrate rate limiting that works under real traffic patterns
- Add request IDs and basic metrics for observability
- Handle graceful shutdown correctly
- Keep the codebase simple, readable, and extensible

This project is intentionally scoped to show strong fundamentals in Rust async programming and backend system design.

## Planned Features

- HTTP server using Axum
- Request ID middleware
- Sliding window rate limiting middleware
- In memory metrics counters
- Graceful shutdown handling
- Clean separation of layers and modules

## Tech Stack

- Rust
- Axum
- Tokio
- Tower (middleware patterns)

## Project Status

Initial setup complete.  
Core server and middleware will be implemented step by step.

## How This Project Is Built

This repository starts from a minimal scaffold and grows incrementally. Each feature is implemented fully and tested locally to show correct behavior rather than partial or placeholder logic.

## License

MIT