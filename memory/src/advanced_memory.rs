//! Advanced Memory Management Examples
//!
//! This module demonstrates more complex memory management patterns in Rust:
//! - Custom allocators
//! - Memory pools
//! - Zero-copy operations
//! - Memory-mapped files
//! - Unsafe pointer operations

use std::alloc::{System, GlobalAlloc, Layout};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Custom allocator that tracks allocation statistics
pub struct TrackingAllocator {
    allocations: AtomicUsize,
    deallocations: AtomicUsize,
    total_allocated: AtomicUsize,
}

impl TrackingAllocator {
    pub const fn new() -> Self {
        Self {
            allocations: AtomicUsize::new(0),
            deallocations: AtomicUsize::new(0),
            total_allocated: AtomicUsize::new(0),
        }
    }

    pub fn stats(&self) -> (usize, usize, usize) {
        (
            self.allocations.load(Ordering::Relaxed),
            self.deallocations.load(Ordering::Relaxed),
            self.total_allocated.load(Ordering::Relaxed),
        )
    }
}

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            self.allocations.fetch_add(1, Ordering::Relaxed);
            self.total_allocated.fetch_add(layout.size(), Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        self.deallocations.fetch_add(1, Ordering::Relaxed);
        self.total_allocated.fetch_sub(layout.size(), Ordering::Relaxed);
    }
}

/// Simple memory pool implementation for fixed-size allocations
pub struct MemoryPool {
    block_size: usize,
    pool: Vec<u8>,
    free_list: Vec<*mut u8>,
}

impl MemoryPool {
    pub fn new(block_size: usize, capacity: usize) -> Self {
        let pool = vec![0u8; block_size * capacity];
        let mut free_list = Vec::with_capacity(capacity);

        // Initialize free list with pointers to each block
        for i in 0..capacity {
            let ptr = unsafe { pool.as_ptr().add(i * block_size) as *mut u8 };
            free_list.push(ptr);
        }

        Self {
            block_size,
            pool,
            free_list,
        }
    }

    pub fn allocate(&mut self) -> Option<*mut u8> {
        self.free_list.pop()
    }

    pub fn deallocate(&mut self, ptr: *mut u8) {
        // Simple validation - ensure pointer is within our pool range
        let pool_start = self.pool.as_ptr() as usize;
        let pool_end = pool_start + self.pool.len();
        let ptr_addr = ptr as usize;

        if ptr_addr >= pool_start && ptr_addr < pool_end {
            self.free_list.push(ptr);
        }
    }

    pub fn available_blocks(&self) -> usize {
        self.free_list.len()
    }

    pub fn block_size(&self) -> usize {
        self.block_size
    }
}

/// Demonstrates zero-copy operations using slices
pub mod zero_copy {
    /// Process data without copying by using slices
    pub fn process_data_slice(data: &[u8]) -> u64 {
        let mut sum = 0u64;
        for &byte in data {
            sum += byte as u64;
        }
        sum
    }

    /// Split data into multiple views without copying
    pub fn split_data_view(data: &[u8]) -> (&[u8], &[u8]) {
        let mid = data.len() / 2;
        data.split_at(mid)
    }
}

/// Advanced pointer manipulation examples
pub mod advanced_pointers {

    /// Safe pointer arithmetic example
    pub fn pointer_arithmetic_example() {
        let mut array = [1, 2, 3, 4, 5];
        let base_ptr = array.as_mut_ptr();

        unsafe {
            // Access elements using pointer arithmetic
            for i in 0..array.len() {
                let element_ptr = base_ptr.add(i);
                println!("Element at index {}: {}", i, *element_ptr);
            }
        }
    }

    /// Example of working with raw pointers
    pub fn raw_pointer_example() {
        let value = 42i32;
        let raw_ptr = &value as *const i32;
        let _raw_mut_ptr = &value as *const i32 as *mut i32;

        println!("Raw pointer address: {:p}", raw_ptr);

        unsafe {
            println!("Value through raw pointer: {}", *raw_ptr);

            // Note: This would be undefined behavior since value is immutable
            // *_raw_mut_ptr = 100; // Don't do this!
        }
    }

    /// Example of creating and working with custom smart pointers
    pub struct MySmartPointer<T> {
        data: *mut T,
    }

    impl<T> MySmartPointer<T> {
        pub fn new(value: T) -> Self {
            let boxed = Box::new(value);
            Self {
                data: Box::into_raw(boxed),
            }
        }

        pub fn get(&self) -> &T {
            unsafe { &*self.data }
        }
    }

    impl<T> Drop for MySmartPointer<T> {
        fn drop(&mut self) {
            unsafe {
                let _ = Box::from_raw(self.data);
            }
        }
    }

    pub fn custom_smart_pointer_example() {
        let smart_ptr = MySmartPointer::new(42);
        println!("Value through custom smart pointer: {}", smart_ptr.get());
    }
}

/// Memory alignment and padding examples
pub mod memory_alignment {
    use std::mem;

    #[repr(C)]
    struct UnoptimizedStruct {
        a: u8,    // 1 byte + 3 bytes padding
        b: u32,   // 4 bytes
        c: u16,   // 2 bytes + 2 bytes padding
        d: u64,   // 8 bytes
    }

    #[repr(C)]
    struct OptimizedStruct {
        b: u32,   // 4 bytes
        d: u64,   // 8 bytes
        c: u16,   // 2 bytes
        a: u8,    // 1 byte + 1 byte padding
    }

    pub fn demonstrate_alignment() {
        println!("=== Memory Alignment Examples ===");

        println!("Unoptimized struct size: {} bytes", mem::size_of::<UnoptimizedStruct>());
        println!("Optimized struct size: {} bytes", mem::size_of::<OptimizedStruct>());

        println!("Unoptimized struct alignment: {} bytes", mem::align_of::<UnoptimizedStruct>());
        println!("Optimized struct alignment: {} bytes", mem::align_of::<OptimizedStruct>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracking_allocator_stats() {
        let allocator = TrackingAllocator::new();

        // Note: We can't test the global allocator directly in a test
        // since we already have one defined in the main function.
        // This test just verifies the statistics functionality.
        let (allocs, _deallocs, total) = allocator.stats();

        assert_eq!(allocs, 0);
        assert_eq!(total, 0);
    }

    #[test]
    fn test_memory_pool() {
        let mut pool = MemoryPool::new(64, 10);
        assert_eq!(pool.available_blocks(), 10);

        let ptr1 = pool.allocate();
        assert!(ptr1.is_some());
        assert_eq!(pool.available_blocks(), 9);

        let ptr2 = pool.allocate();
        assert!(ptr2.is_some());
        assert_eq!(pool.available_blocks(), 8);

        if let Some(ptr) = ptr1 {
            pool.deallocate(ptr);
        }
        assert_eq!(pool.available_blocks(), 9);
    }

    #[test]
    fn test_zero_copy() {
        let data = vec![1, 2, 3, 4, 5];
        let sum = zero_copy::process_data_slice(&data);
        assert_eq!(sum, 15);

        let (first, second) = zero_copy::split_data_view(&data);
        assert_eq!(first, &[1, 2]);
        assert_eq!(second, &[3, 4, 5]);
    }
}

pub fn run_advanced_examples() {
    println!("\n🚀 Advanced Memory Management Examples");
    println!("=====================================");

    // Custom allocator example
    println!("\n--- Custom Allocator ---");
    #[global_allocator]
    static TRACKING_ALLOC: TrackingAllocator = TrackingAllocator::new();

    let _data = vec![1u8; 1024];
    let (allocs, deallocs, total) = TRACKING_ALLOC.stats();
    println!("Allocations: {}, Deallocations: {}, Total: {} bytes", allocs, deallocs, total);

    // Memory pool example
    println!("\n--- Memory Pool ---");
    let mut pool = MemoryPool::new(32, 5);
    println!("Initial available blocks: {}", pool.available_blocks());

    let ptr = pool.allocate();
    println!("After allocation: {} blocks available", pool.available_blocks());

    if let Some(p) = ptr {
        pool.deallocate(p);
        println!("After deallocation: {} blocks available", pool.available_blocks());
    }

    // Zero-copy operations
    println!("\n--- Zero-Copy Operations ---");
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let (first_half, second_half) = zero_copy::split_data_view(&data);
    println!("Original data: {:?}", data);
    println!("First half: {:?}", first_half);
    println!("Second half: {:?}", second_half);

    // Advanced pointers
    println!("\n--- Advanced Pointer Operations ---");
    advanced_pointers::pointer_arithmetic_example();
    advanced_pointers::custom_smart_pointer_example();

    // Memory alignment
    memory_alignment::demonstrate_alignment();
}