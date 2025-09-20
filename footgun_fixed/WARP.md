# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

The `footgun_fixed` project is the corrected version of the original "footgun" example, demonstrating proper concurrent programming techniques in Rust. It shows how to safely handle shared state across multiple threads using atomic operations and mutexes.

### Architecture

This is a single-binary Rust project that demonstrates two safe concurrency solutions:

**Solution 1: Atomic Operations (AtomicI32)**
- Lock-free concurrent operations
- High performance with minimal overhead  
- Uses `std::sync::atomic::AtomicI32` for thread-safe counting
- Suitable for simple operations like counters

**Solution 2: Mutual Exclusion (Mutex)**
- Thread-safe access through locking mechanisms
- Uses `Arc<Mutex<i32>>` for shared ownership and exclusive access
- Better suited for complex critical sections
- Slightly higher overhead due to locking

### Performance Comparison
The program runs both solutions and provides timing comparisons, typically showing atomic operations are faster than mutex-based approaches for simple increment operations.

## Common Development Commands

### Building and Checking
```bash
# Check for compilation errors
cargo check

# Check with clippy linting
cargo clippy

# Check the entire workspace (run from parent directory)
cargo check --workspace
```

### Testing
```bash
# Run tests
cargo test

# Run tests for entire workspace (from parent directory)
cargo test --workspace
```

### Running
```bash
# Run the concurrent counter demonstration
cargo run

# Run with release optimizations for better performance comparison
cargo run --release
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
- `src/main.rs` - Contains both atomic and mutex-based solutions with performance comparison
- `Cargo.toml` - Project configuration using Rust 2024 edition

### Expected Behavior
- Both solutions will always produce the correct result: 1,000,000
- Atomic version typically runs faster than mutex version
- Results are deterministic and thread-safe
- Performance difference varies by system but atomic is usually 2-5x faster

### Educational Purpose
This project demonstrates:
- Proper concurrent programming techniques in Rust
- When to use atomic types vs mutexes
- Performance trade-offs between different synchronization primitives
- Safe alternatives to unsafe concurrent code
- How Rust's type system prevents data races at compile time

### Key Concepts Illustrated
- **AtomicI32**: Lock-free atomic operations with memory ordering
- **Arc (Atomically Reference Counted)**: Safe shared ownership across threads
- **Mutex**: Mutual exclusion for protecting critical sections
- **Thread safety**: Both solutions guarantee no data races
- **Performance trade-offs**: Atomic vs locking overhead

### Workspace Context
This project is part of the `ultimate_rust` learning workspace. It serves as the "correct" counterpart to the `footgun` project, showing how to properly implement the same functionality safely.
