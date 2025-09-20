# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

The `footgun` project is a demonstration of unsafe Rust code that highlights data race conditions and undefined behavior. It's part of the `ultimate_rust` workspace and serves as an educational example of what NOT to do in concurrent Rust programming.

### Architecture

This is a single-binary Rust project that demonstrates:
- Unsafe static mutable global state (`COUNTER`)
- Concurrent access without proper synchronization
- Data races across 1000 threads, each incrementing a shared counter 1000 times
- Undefined behavior due to unsynchronized access to mutable static data

The project intentionally contains unsafe code and will not compile with Rust 2024 edition due to stricter static reference rules.

## Common Development Commands

### Building and Checking
```bash
# Check for compilation errors (will fail due to unsafe static refs)
cargo check

# Check with clippy linting (will also fail)
cargo clippy

# Check the entire workspace (run from parent directory)
cargo check --workspace
```

### Testing
```bash
# Run tests (will fail to compile)
cargo test

# Run tests for entire workspace (from parent directory)
cargo test --workspace
```

### Running
```bash
# This project intentionally does not compile with current Rust edition
# To run it, you would need to allow unsafe static references or use older Rust
cargo run
```

### Workspace Commands (from parent directory)
```bash
# Check all workspace members
cargo check --workspace

# Build all workspace members
cargo build --workspace

# Clean all workspace artifacts
cargo clean --workspace
```

## Development Notes

### Key Files
- `src/main.rs` - Contains the unsafe concurrent code demonstration
- `Cargo.toml` - Project configuration using Rust 2024 edition

### Expected Behavior
- The project **intentionally fails to compile** with Rust 2024 edition
- It demonstrates undefined behavior through unsynchronized access to static mutable data
- If it could run, it would show non-deterministic results due to data races

### Educational Purpose
This project serves as a learning tool to understand:
- Why Rust's safety guarantees exist
- The dangers of `unsafe` code without proper synchronization
- Race conditions in multithreaded programs
- The importance of proper concurrency primitives

### Workspace Context
This project is part of the `ultimate_rust` learning workspace containing multiple Rust example projects. When working on this project, be aware that:
- It's one of many educational examples
- The parent directory contains the workspace `Cargo.toml`
- Other workspace members demonstrate proper Rust patterns