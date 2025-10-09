/*
 * Rust 内存管理深入学习示例
 *
 * 本项目展示了 Rust 中各种内存管理技术，从基础概念到高级应用，
 * 包含详细的中文注释、实用示例和最佳实践指导。
 *
 * 📚 官方文档链接：
 *
 * 🔰 基础概念
 * 1. Rust Book - 所有权系统:
 *    https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
 *
 * 2. Rust Book - 借用和切片:
 *    https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
 *
 * 3. Rust Book - 生命周期:
 *    https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
 *
 * ⚙️ 内存管理
 * 4. std::alloc 模块:
 *    https://doc.rust-lang.org/std/alloc/index.html
 *
 * 5. 智能指针详解:
 *    https://doc.rust-lang.org/book/ch15-00-smart-pointers.html
 *
 * 6. 不安全 Rust:
 *    https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
 *
 * 🚀 高级概念
 * 7. Rustonomicon (不安全 Rust 指南):
 *    https://doc.rust-lang.org/nomicon/
 *
 * 8. 全局分配器:
 *    https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html
 *
 * 9. 内存布局和对齐:
 *    https://doc.rust-lang.org/reference/type-layout.html
 *
 * 10. C 互操作:
 *     https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#calling-c-functions-from-rust
 */

use libc; // C 库接口，用于 C 风格的内存分配

/// 演示使用 libc 进行 C 风格的内存分配
///
/// 这个函数展示了如何在 Rust 中使用 C 库函数进行内存管理：
/// - 使用 malloc 分配内存
/// - 手动写入和读取内存
/// - 使用 free 释放内存
/// - 展示了 Rust 与 C 代码的互操作性
///
/// ⚠️ 重要提示：在 Rust 中通常应该使用 std::alloc 或智能指针，
/// 这个例子主要用于演示与 C 代码的互操作
fn allocate_memory_with_libc() {
    println!("=== 使用 libc 进行 C 风格内存分配 ===");

    unsafe {
        // 使用 malloc 为 i32 分配内存
        // malloc 返回 void*，需要转换为具体的类型指针
        let my_num = libc::malloc(std::mem::size_of::<i32>()) as *mut i32;

        // 检查分配是否成功
        // malloc 在内存不足时会返回 NULL
        if my_num.is_null() {
            panic!("内存分配失败");
        }

        println!("为 i32 分配了 {} 字节，地址: {:p}",
                std::mem::size_of::<i32>(), my_num);

        // 向分配的内存写入值
        // 通过解引用裸指针来访问内存
        *my_num = 42;
        println!("向分配的内存写入值: {}", *my_num);

        // 读取并验证值
        assert_eq!(42, *my_num);
        println!("验证值: {}", *my_num);

        // 释放分配的内存
        // 必须使用 free 释放 malloc 分配的内存
        libc::free(my_num as *mut std::ffi::c_void);
        println!("内存释放成功");
    }
}

/// 演示使用 Rust 的 std::alloc 进行手动内存分配
///
/// 这个函数展示了 Rust 原生的内存分配方式：
/// - 使用 Layout 描述内存布局
/// - 使用 alloc 分配内存
/// - 使用 dealloc 释放内存
/// - 比 libc 更符合 Rust 的惯用做法
///
/// 优点：
/// - 类型安全的 Layout 系统
/// - 更好的错误处理
/// - 与 Rust 生态系统集成更好
fn allocate_memory_with_rust() {
    println!("\n=== 使用 Rust std::alloc 进行手动内存分配 ===");

    use std::alloc::{Layout, alloc, dealloc}; // 内存分配模块

    unsafe {
        // 为 u32 创建内存布局
        // Layout 包含了类型的大小和对齐要求
        let layout = Layout::new::<u32>();
        println!("为 u32 创建内存布局: {} 字节", layout.size());

        // 分配内存
        // alloc 返回 *mut u8，需要根据布局进行类型转换
        let ptr = alloc(layout);
        println!("分配内存地址: {:p}", ptr);

        // 写入值
        // 需要将指针转换为正确的类型
        *(ptr as *mut u32) = 42;
        println!("向分配的内存写入值: {}", *(ptr as *mut u32));

        // 读取并验证
        assert_eq!(42, *(ptr as *mut u32));
        println!("验证值: {}", *(ptr as *mut u32));

        // 释放内存
        // 必须传入相同的 layout 以确保正确释放
        dealloc(ptr, layout);
        println!("内存释放成功");
    }
}

/// 演示栈分配与堆分配的区别
///
/// 这个函数展示了 Rust 中两种主要的内存分配方式：
/// - 栈分配：快速、自动清理、固定大小
/// - 堆分配：较慢、手动管理、动态大小
///
/// 选择原则：
/// - 大小已知且较小 → 栈分配
/// - 大小未知或较大 → 堆分配
/// - 需要在作用域外存在 → 堆分配
fn demonstrate_stack_vs_heap() {
    println!("\n=== 栈分配与堆分配对比 ===");

    // 栈分配 - 快速、自动清理
    // 局部变量默认在栈上分配
    // 在作用域结束时自动释放
    let stack_var = 100;
    println!("栈变量: {} (存储在栈上)", stack_var);

    // 堆分配使用 Box - 较慢、需要清理
    // Box 将数据分配到堆上，栈上只保存指针
    let heap_var = Box::new(200);
    println!("堆变量: {} (存储在堆上，地址: {:p})", heap_var, heap_var);

    // Vector 演示集合的堆分配
    // Vector 本身在栈上，但元素存储在堆上
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("堆上的 Vector: {:?} (容量: {})", vec, vec.capacity());

    // 内存使用情况对比
    println!("\n内存使用对比:");
    println!("  栈变量大小: {} 字节", std::mem::size_of_val(&stack_var));
    println!("  Box 指针大小: {} 字节 (数据在堆上)", std::mem::size_of_val(&heap_var));
    println!("  Vec 头部大小: {} 字节 (元素在堆上)", std::mem::size_of_val(&vec));
}

/// 演示内存布局和大小信息
///
/// 这个函数展示了 Rust 中各种类型的内存特性：
/// - 类型大小 (size_of)
/// - 内存对齐 (align_of)
/// - 指针类型大小
///
/// 理解内存布局对于性能优化和 FFI (Foreign Function Interface) 很重要
fn demonstrate_memory_layout() {
    println!("\n=== 内存布局信息 ===");

    // 基本类型大小
    println!("各种类型的大小:");
    println!("  i32: {} 字节", std::mem::size_of::<i32>());
    println!("  i64: {} 字节", std::mem::size_of::<i64>());
    println!("  f64: {} 字节", std::mem::size_of::<f64>());
    println!("  bool: {} 字节", std::mem::size_of::<bool>());
    println!("  char: {} 字节", std::mem::size_of::<char>());

    // 指针类型大小
    println!("\n指针类型大小:");
    println!("  Box<i32>: {} 字节", std::mem::size_of::<Box<i32>>());
    println!("  &i32: {} 字节", std::mem::size_of::<&i32>());
    println!("  *mut i32: {} 字节", std::mem::size_of::<*mut i32>());
    println!("  Vec<i32>: {} 字节", std::mem::size_of::<Vec<i32>>());

    // 内存对齐信息
    // 对齐决定了类型在内存中的起始地址
    println!("\n各种类型的对齐要求:");
    println!("  i32: {} 字节对齐", std::mem::align_of::<i32>());
    println!("  i64: {} 字节对齐", std::mem::align_of::<i64>());
    println!("  f64: {} 字节对齐", std::mem::align_of::<f64>());

    // 复杂类型示例
    #[repr(C)] // C 布局，用于 FFI
    struct ExampleStruct {
        a: i8,
        b: i32,
        c: i16,
    }

    #[repr(packed)] // 紧凑布局，无填充
    struct PackedStruct {
        a: i8,
        b: i32,
        c: i16,
    }

    println!("\n结构体布局对比:");
    println!("  C布局结构体: {} 字节", std::mem::size_of::<ExampleStruct>());
    println!("  紧凑布局结构体: {} 字节", std::mem::size_of::<PackedStruct>());
}

/// 演示使用智能指针的安全内存模式
///
/// 这个函数展示了 Rust 中各种智能指针的用途：
/// - Box<T>: 单所有权堆分配
/// - Rc<T>: 引用计数，多所有权（单线程）
/// - Arc<T>: 原子引用计数，多所有权（多线程）
///
/// 智能指针的优点：
/// - 自动内存管理
/// - 防止内存泄漏
/// - 类型安全
fn demonstrate_smart_pointers() {
    println!("\n=== 智能指针和内存安全 ===");

    // Box<T> - 堆分配，单一所有权
    // 最简单的智能指针，提供堆分配
    let boxed_value = Box::new(42);
    println!("Box 值: {} (栈上指针大小: {} 字节)",
             boxed_value, std::mem::size_of_val(&boxed_value));

    // Rc<T> - 引用计数，多个所有者（单线程）
    // 允许多个变量共享同一数据的所有权
    use std::rc::Rc;
    let rc_value = Rc::new(100);
    let rc_clone1 = Rc::clone(&rc_value);
    let rc_clone2 = Rc::clone(&rc_value);
    println!("Rc 值: {} (强引用计数: {})",
             rc_value, Rc::strong_count(&rc_value));

    // 释放一个克隆，引用计数减少
    drop(rc_clone1);
    println!("释放一个克隆后，强引用计数: {}", Rc::strong_count(&rc_value));

    // Arc<T> - 原子引用计数，多线程安全
    // 类似于 Rc，但可以在多线程间安全共享
    use std::sync::Arc;
    let arc_value = Arc::new(200);
    let arc_clone = Arc::clone(&arc_value);
    println!("Arc 值: {} (强引用计数: {})",
             arc_value, Arc::strong_count(&arc_value));

    // 智能指针的内存使用对比
    println!("\n智能指针内存使用:");
    println!("  Box<i32>: {} 字节", std::mem::size_of::<Box<i32>>());
    println!("  Rc<i32>: {} 字节", std::mem::size_of::<Rc<i32>>());
    println!("  Arc<i32>: {} 字节", std::mem::size_of::<Arc<i32>>());
}

// 导入高级内存管理示例模块
mod advanced_memory;

// 主函数：运行所有内存管理演示
fn main() {
    println!("🦀 Rust 内存管理演示");
    println!("============================");

    // 运行所有基础演示
    allocate_memory_with_libc();      // C 风格内存分配
    allocate_memory_with_rust();      // Rust 原生内存分配
    demonstrate_stack_vs_heap();      // 栈与堆分配对比
    demonstrate_memory_layout();      // 内存布局信息
    demonstrate_smart_pointers();     // 智能指针演示

    // 运行高级示例
    advanced_memory::run_advanced_examples();

    println!("\n✅ 所有内存管理示例完成！");
    println!("\n📚 延伸阅读:");
    println!("  • Rust Book - 所有权: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html");
    println!("  • Rustonomicon: https://doc.rust-lang.org/nomicon/");
    println!("  • std::alloc 文档: https://doc.rust-lang.org/std/alloc/index.html");
    println!("  • 不安全 Rust 指南: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html");
    println!("  • 自定义分配器: https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html");
    println!("  • 智能指针指南: https://doc.rust-lang.org/book/ch15-00-smart-pointers.html");
    println!("  • 内存布局详解: https://doc.rust-lang.org/reference/type-layout.html");
}
