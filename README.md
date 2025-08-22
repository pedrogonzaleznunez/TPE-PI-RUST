# TPE PI ITBA (Rust Edition)

This is a Rust implementation of the July 2025 Imperative Programming final project at ITBA.

## Overview

While the Imperative Programming course at ITBA focuses exclusively on the C programming language, the final project presents a great medium-sized programming challenge that can be tackled in any language. We chose to implement it in Rust to explore Rust's features, abstractions, and build tooling.

This project represents our solution to the final assignment for the Imperative Programming course, reimagined in Rust. It demonstrates how the same algorithmic thinking and problem-solving skills learned in C can be applied in any language, and how using more advanced data structures and zero cost abstractions can improve performance and developer experience.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- Cargo (comes with Rust)

### Building and Running

```bash
# Clone the repository
git clone https://github.com/pedrogonzaleznunez/TPE-PI-RUST.git
cd TPE-PI-RUST

# Build the project
cargo build --features [nyc|chi]

# Run the project
cargo run --features [nyc|chi]

# Run tests
cargo test --features [nyc|chi]
```

## Development

This project follows conventional commit standards. See the `.githooks/` directory for commit message validation.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes following conventional commits
4. Submit a pull request
