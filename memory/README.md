# Rust Memory Management Examples

This project demonstrates various memory management techniques in Rust, from basic concepts to advanced patterns. It serves as a comprehensive learning resource for understanding how Rust handles memory allocation, deallocation, and safety.

## 🚀 Features Demonstrated

### Basic Memory Management
- **C-style memory allocation** using `libc`
- **Rust's manual allocation** using `std::alloc`
- **Stack vs heap allocation** patterns
- **Memory layout and alignment** information
- **Smart pointers** (Box, Rc, Arc)

### Advanced Memory Management
- **Custom allocators** with allocation tracking
- **Memory pools** for fixed-size allocations
- **Zero-copy operations** using slices
- **Raw pointer manipulation** and arithmetic
- **Custom smart pointer** implementation
- **Memory alignment** optimization

## 📁 Project Structure

```
memory/
├── src/
│   ├── main.rs              # Main demonstrations and examples
│   └── advanced_memory.rs   # Advanced memory management patterns
├── Cargo.toml               # Project configuration
└── README.md               # This file
```

## 🛠️ Running the Examples

```bash
# Run all memory management examples
cargo run

# Run tests
cargo test

# Build with optimizations
cargo build --release
```

## 📚 Topics Covered

### 1. C-style Memory Allocation with libc
Learn how to interface with C memory management functions:
- `malloc()` for memory allocation
- `free()` for memory deallocation
- Memory safety checks in unsafe blocks

### 2. Rust Manual Memory Allocation
Use Rust's built-in allocation system:
- `Layout` for memory layout specification
- `alloc()` for manual allocation
- `dealloc()` for manual deallocation

### 3. Stack vs Heap Allocation
Understand the differences:
- **Stack**: Fast, automatic cleanup, fixed size
- **Heap**: Slower, manual cleanup, dynamic size
- **Box**: Smart pointer for heap allocation
- **Vec**: Dynamic collections on heap

### 4. Smart Pointers
Master Rust's ownership system:
- **Box<T>**: Single ownership, heap allocation
- **Rc<T>**: Multiple owners, single-threaded
- **Arc<T>**: Multiple owners, multi-threaded

### 5. Custom Allocators
Build your own allocator:
- Track allocation statistics
- Implement `GlobalAlloc` trait
- Monitor memory usage patterns

### 6. Memory Pools
Optimize allocation patterns:
- Fixed-size block allocation
- Fast allocation/deallocation
- Memory fragmentation reduction

### 7. Zero-Copy Operations
Minimize memory copying:
- Use slices for data views
- Split data without copying
- Efficient data processing

### 8. Raw Pointer Operations
Work with unsafe Rust:
- Pointer arithmetic
- Memory addresses
- Custom smart pointers

### 9. Memory Alignment
Optimize memory layout:
- Structure padding
- Alignment requirements
- Performance optimization

## 🔗 Useful Links

### Official Documentation
- **Rust Book - Ownership**: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
- **Rustonomicon**: https://doc.rust-lang.org/nomicon/
- **std::alloc module**: https://doc.rust-lang.org/std/alloc/index.html
- **Unsafe Rust**: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
- **Smart Pointers**: https://doc.rust-lang.org/book/ch15-00-smart-pointers.html
- **Custom Allocators**: https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html

### External Resources
- **libc crate**: https://docs.rs/libc/latest/libc/
- **Rust Memory Safety**: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
- **Zero-Copy Programming**: https://github.com/rust-lang/rfcs/blob/master/text/2414-dropck.md

## 🎯 Learning Outcomes

After studying these examples, you will understand:

1. **Memory allocation patterns** in Rust vs C
2. **Stack vs heap allocation** trade-offs
3. **Smart pointer usage** and ownership
4. **Unsafe Rust** for low-level memory operations
5. **Custom allocator** implementation
6. **Memory pool** design patterns
7. **Zero-copy** programming techniques
8. **Memory alignment** and optimization

## ⚠️ Safety Notes

This project includes `unsafe` code blocks that demonstrate low-level memory operations. These are for educational purposes only and should be used with caution in production code. Always ensure that unsafe code follows Rust's safety guidelines.

## 🧪 Testing

The project includes comprehensive tests:
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_tracking_allocator

# Run tests with output
cargo test -- --nocapture
```

## 📈 Performance Considerations

The examples demonstrate various performance trade-offs:
- **Stack allocation**: Fastest but limited scope
- **Heap allocation**: Flexible but slower
- **Memory pools**: Good for frequent allocations
- **Custom allocators**: Can optimize specific patterns

## 🤝 Contributing

This is a learning project. Feel free to:
- Add new memory management examples
- Improve documentation
- Add more performance benchmarks
- Create additional learning exercises

## 📄 License

This project is for educational purposes and follows Rust's documentation licensing.

---

**Happy learning! 🦀**