# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is the `ultimate_rust` workspace - a comprehensive Rust learning repository containing 40+ individual projects that progressively teach Rust concepts from basics to advanced concurrent programming and async patterns. This is structured as a Cargo workspace with each concept as a separate package.

## Common Development Commands

### Workspace Operations
```bash
# Check entire workspace
cargo check --workspace

# Build all projects
cargo build --workspace

# Run tests across workspace
cargo test --workspace

# Run clippy on all packages
cargo clippy --workspace

# Clean all artifacts
cargo clean --workspace
```

### Individual Project Operations
```bash
# Navigate to specific project and run
cd <project_name>
cargo run

# Run in release mode (critical for performance testing)
cargo run --release

# Check specific project
cargo check

# Test specific project
cargo test
```

### Performance Testing
```bash
# For concurrent projects, always test with release mode
cargo run --release

# Time execution for performance analysis
time cargo run --release

# Multiple runs for consistency testing
for i in {1..5}; do echo "Run $i:"; cargo run --release; done
```

## Architecture and Learning Path

### Progressive Learning Structure

**Foundation Projects**: `hello_world`, `variables` - Basic Rust syntax and concepts

**Authentication System**: `authentication` (library), `login`, `login_manager` - JSON handling with serde, user management, password hashing with SHA-256

**Concurrency Fundamentals**: `threads_demo`, `divide_work`, `my_thread`, `scoped_thread` - Thread basics and lifecycle management

**Thread Safety Learning Path**:
- `footgun`: Intentionally unsafe code with data races (fails to compile in Rust 2024)
- `footgun_fixed`: Proper solutions using atomic operations and mutexes
- `footgun_manual`: Practice environment with comprehensive Makefile tooling

**Synchronization Primitives**: `mutex_demo`, `read_write_lock_demo`, `deadlocks`, `parking` - Various locking mechanisms and common pitfalls

**Advanced Threading**: `channels1`, `threadpool_workers`, `work_queue`, `cpu_affinity` - Communication patterns and thread management

**Performance Computing**: `rayon_iters`, `rayon_scopes` - Data parallelism using Rayon

**Async Programming**: Multiple async projects including:
- `hello_async`, `hello_tokio` - Async runtime basics
- `tokio_test`, `blocking`, `errors` - Error handling and blocking operations
- `tokio_channels`, `tokio_share_state`, `tokio_select` - Advanced async patterns
- `tokio_db` - Database operations with SQLx
- `tokio_web_service` - Web service implementation

**Networking**: `tcpserver`, `tokio_tcpclient`, `restclient` - Network programming examples

**File I/O**: `fileio`, `thumbs` (image processing) - File system operations

### Key Educational Patterns

**Broken → Fixed → Manual Trilogy**: The footgun projects demonstrate:
1. Unsafe patterns showing data races
2. Correct atomic/mutex solutions with performance comparisons
3. Hands-on practice environment with comprehensive tooling

**Bilingual Documentation**: Many projects include extensive Chinese comments for better learning comprehension.

### Special Project Commands

#### Footgun Manual Project (Advanced Learning Tool)
```bash
cd footgun_manual
make help                    # Show all available commands
make run                     # Run your fixed version
make optimized              # Run optimized reference implementation
make compare                # Compare all three versions
make validate               # Verify fix correctness
make bench                  # Performance benchmarking
```

#### Authentication Projects
```bash
cd login_manager
cargo run                   # Auto-creates users.json file
rm users.json && cargo run  # Reset user database
```

## Development Environment

### Workspace Structure
- **Root Cargo.toml**: Defines workspace members, uses Rust 2021 edition
- **Individual Cargo.toml**: Each project has its own dependencies (some use Rust 2024)
- **src/main.rs**: Main entry point (except `authentication` which is a library)

### Key Dependencies
- **serde/serde_json**: JSON serialization in authentication projects
- **tokio**: Async runtime (featured in many projects)
- **rayon**: Data parallelism
- **sqlx**: Database operations
- **sha2**: Password hashing
- **anyhow**: Error handling

### Important Considerations
- **Release Mode**: Performance-critical examples require `--release` flag
- **Multiple Runs**: Concurrent programs should be tested multiple times
- **Platform Differences**: Thread/CPU affinity features may vary across platforms
- **Compilation Issues**: Some projects intentionally fail to compile to demonstrate safety

### Testing Strategy
- Unit tests in authentication library
- Integration testing through multiple program runs
- Performance validation through benchmarking
- Consistency verification for concurrent programs

### Key Files for Understanding
- `NOTES.md`: Development environment setup
- `CODEBUDDY.md`: Comprehensive Chinese guide
- `WARP.md`: English project documentation
- `footgun_manual/Makefile`: Build automation example
- Individual project READMEs in specific directories

This workspace serves as a hands-on laboratory for learning Rust's approach to systems programming, with special emphasis on safe concurrent programming patterns and progressive skill-building.