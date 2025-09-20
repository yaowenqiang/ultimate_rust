# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

The `footgun_manual` project is a copy of the original "footgun" example that you can use to practice manual data race fixes. This serves as a hands-on learning exercise for understanding concurrent programming in Rust.

### Your Mission 🎯

Fix the data race in `src/main.rs` by implementing safe concurrent programming patterns. You have multiple options:

1. **Atomic Operations** - Use `std::sync::atomic::AtomicI32`
2. **Mutex** - Use `Arc<Mutex<i32>>` for thread-safe shared state
3. **Channel-based Communication** - Use `std::sync::mpsc` channels
4. **Other Sync Primitives** - Explore `RwLock`, `Barrier`, etc.

### Current Problem 🚨

The code contains:
- Unsafe static mutable global state (`COUNTER`)
- Concurrent access without proper synchronization  
- Data races across 1000 threads
- Non-deterministic results due to race conditions

### Goal 🏆

Make the program:
- ✅ Compile successfully
- ✅ Always produce the correct result: 1,000,000
- ✅ Be thread-safe and deterministic
- ✅ Follow Rust best practices

## Common Development Commands

### Building and Checking
```bash
# Check for compilation errors
cargo check

# Check with clippy for additional warnings
cargo clippy

# Format your code
cargo fmt
```

### Testing Your Solution
```bash
# Run your fixed version
cargo run

# Run multiple times to verify consistency
for i in {1..5}; do echo "Run $i:"; cargo run --release; done

# Compare with the broken version
cd ../footgun && cargo run
```

### Benchmarking
```bash
# Run in release mode for performance testing
cargo run --release

# Time the execution
time cargo run --release
```

## Hints and Tips 💡

### Option 1: Atomic Approach
```rust
// Replace the unsafe static with:
use std::sync::atomic::{AtomicI32, Ordering};
static COUNTER: AtomicI32 = AtomicI32::new(0);

// In your threads, use:
COUNTER.fetch_add(1, Ordering::SeqCst);
```

### Option 2: Mutex Approach  
```rust
// Use Arc for shared ownership, Mutex for safe access:
use std::sync::{Arc, Mutex};
let counter = Arc::new(Mutex::new(0));

// Clone the Arc for each thread:
let counter_clone = Arc::clone(&counter);
// In thread: *counter_clone.lock().unwrap() += 1;
```

### Option 3: Channel Approach
```rust
// Collect results via channels:
use std::sync::mpsc;
let (tx, rx) = mpsc::channel();
// Each thread sends results, main thread sums them
```

### Things to Consider
- **Performance**: Atomic operations are usually fastest for simple counters
- **Complexity**: Mutexes are better for complex critical sections
- **Scalability**: Consider contention under high thread counts
- **Memory Ordering**: Understand `Ordering::SeqCst` vs other orderings

## Validation Steps ✅

1. **Compile Check**: `cargo check` should pass
2. **Correctness**: Result should always be exactly 1,000,000
3. **Consistency**: Run multiple times - results should be identical
4. **Performance**: Compare timing with other solutions
5. **Code Quality**: Run `cargo clippy` and fix any warnings

## Learning Resources 📚

- [Rust Book - Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [std::sync documentation](https://doc.rust-lang.org/std/sync/index.html)
- [Atomic types](https://doc.rust-lang.org/std/sync/atomic/index.html)
- Compare your solution with `../footgun_fixed/src/main.rs`

## Workspace Context

This is part of the `ultimate_rust` learning workspace:
- `../footgun/` - Original broken version
- `../footgun_fixed/` - Reference implementation
- `../footgun_manual/` - Your practice ground (this project)

Good luck with your manual fix! 🚀
