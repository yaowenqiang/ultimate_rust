//! Memory Management Examples in Rust
//!
//! This demo showcases various memory management techniques in Rust:
//! - Using libc for C-style memory allocation
//! - Using Rust's std::alloc for manual memory management
//! - Stack vs heap allocation patterns
//! - Memory safety with unsafe blocks
//!
//! # Documentation Links:
//! - Rust Book Memory Management: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
//! - std::alloc module: https://doc.rust-lang.org/std/alloc/index.html
//! - Unsafe Rust: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
//! - libc crate: https://docs.rs/libc/latest/libc/

use libc;

/// Demonstrates C-style memory allocation using libc
/// This shows how Rust can interface with C memory management functions
fn allocate_memory_with_libc() {
    println!("=== C-style Memory Allocation with libc ===");

    unsafe {
        // Allocate memory for an i32 using malloc
        let my_num = libc::malloc(std::mem::size_of::<i32>()) as *mut i32;

        // Check if allocation succeeded
        if my_num.is_null() {
            panic!("Failed to allocate memory");
        }

        println!("Allocated {} bytes for i32 at address: {:p}",
                std::mem::size_of::<i32>(), my_num);

        // Write a value to the allocated memory
        *my_num = 42;
        println!("Wrote value {} to allocated memory", *my_num);

        // Read and verify the value
        assert_eq!(42, *my_num);
        println!("Verified value: {}", *my_num);

        // Free the allocated memory
        libc::free(my_num as *mut std::ffi::c_void);
        println!("Memory freed successfully");
    }
}

/// Demonstrates Rust's manual memory allocation using std::alloc
/// This is the more idiomatic Rust approach for manual memory management
fn allocate_memory_with_rust() {
    println!("\n=== Rust Manual Memory Allocation with std::alloc ===");

    use std::alloc::{Layout, alloc, dealloc};

    unsafe {
        // Create a layout for a u32
        let layout = Layout::new::<u32>();
        println!("Created layout for u32: {} bytes", layout.size());

        // Allocate memory
        let ptr = alloc(layout);
        println!("Allocated memory at address: {:p}", ptr);

        // Write a value
        *ptr = 42;
        println!("Wrote value {} to allocated memory", *ptr);

        // Read and verify
        assert_eq!(42, *ptr);
        println!("Verified value: {}", *ptr);

        // Deallocate memory
        dealloc(ptr, layout);
        println!("Memory deallocated successfully");
    }
}

/// Demonstrates stack vs heap allocation differences
fn demonstrate_stack_vs_heap() {
    println!("\n=== Stack vs Heap Allocation ===");

    // Stack allocation - fast, automatic cleanup
    let stack_var = 100;
    println!("Stack variable: {} (stored on stack)", stack_var);

    // Heap allocation using Box - slower, requires cleanup
    let heap_var = Box::new(200);
    println!("Heap variable: {} (stored on heap at {:p})", heap_var, heap_var);

    // Vector demonstrates heap allocation for collections
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("Vector on heap: {:?} (capacity: {})", vec, vec.capacity());
}

/// Demonstrates memory layout and size information
fn demonstrate_memory_layout() {
    println!("\n=== Memory Layout Information ===");

    println!("Size of various types:");
    println!("  i32: {} bytes", std::mem::size_of::<i32>());
    println!("  i64: {} bytes", std::mem::size_of::<i64>());
    println!("  f64: {} bytes", std::mem::size_of::<f64>());
    println!("  Box<i32>: {} bytes", std::mem::size_of::<Box<i32>>());
    println!("  Vec<i32>: {} bytes", std::mem::size_of::<Vec<i32>>());

    // Alignment information
    println!("Alignment of various types:");
    println!("  i32: {} bytes", std::mem::align_of::<i32>());
    println!("  i64: {} bytes", std::mem::align_of::<i64>());
}

/// Demonstrates safe memory patterns using smart pointers
fn demonstrate_smart_pointers() {
    println!("\n=== Smart Pointers and Memory Safety ===");

    // Box<T> - heap allocation with single ownership
    let boxed_value = Box::new(42);
    println!("Boxed value: {}", boxed_value);

    // Rc<T> - reference counting for multiple owners
    use std::rc::Rc;
    let rc_value = Rc::new(100);
    let _rc_clone = Rc::clone(&rc_value);
    println!("Rc value: {}, strong count: {}", rc_value, Rc::strong_count(&rc_value));

    // Arc<T> - atomic reference counting for thread-safe sharing
    use std::sync::Arc;
    let arc_value = Arc::new(200);
    let _arc_clone = Arc::clone(&arc_value);
    println!("Arc value: {}, strong count: {}", arc_value, Arc::strong_count(&arc_value));
}

mod advanced_memory;

fn main() {
    println!("🦀 Rust Memory Management Demo");
    println!("==============================");

    // Run all demonstrations
    allocate_memory_with_libc();
    allocate_memory_with_rust();
    demonstrate_stack_vs_heap();
    demonstrate_memory_layout();
    demonstrate_smart_pointers();

    // Run advanced examples
    advanced_memory::run_advanced_examples();

    println!("\n✅ All memory management examples completed successfully!");
    println!("\n📚 Further Reading:");
    println!("  • Rust Book - Ownership: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html");
    println!("  • Rustonomicon: https://doc.rust-lang.org/nomicon/");
    println!("  • std::alloc docs: https://doc.rust-lang.org/std/alloc/index.html");
    println!("  • Unsafe Rust guidelines: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html");
    println!("  • Custom allocators: https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html");
    println!("  • Smart pointers guide: https://doc.rust-lang.org/book/ch15-00-smart-pointers.html");
}
