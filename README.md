# Ghosted

**Ghosted** is a job application tracker.

---

## Features

- Track job applications and their statuses
- Cross-platform: Linux and macOS

---

## Prerequisites

Before building Ghosted, make sure you have:

- Rust 1.70+ (`rustup` recommended)
- [`cargo-bundle`](https://crates.io/crates/cargo-bundle) for creating distributable packages
- Linux or macOS system

---

## Installation

You can build Ghosted from source using Cargo:

```bash
# Run in debug mode
cargo run

# Run in release mode
cargo run --release
```

## Build & bundle

```bash
cargo install cargo-bundle

cargo bundle --release --target x86_64-apple-darwin

cargo bundle --release --target x86_64-unknown-linux-gnu
```
