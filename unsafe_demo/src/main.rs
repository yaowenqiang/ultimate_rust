/// Rust unsafe 代码演示主函数
///
/// 本程序演示了 Rust 中 unsafe 代码的使用场景和危险性
/// 展示了安全访问与不安全访问向量的区别
fn main() {
    // 创建一个包含 5 个元素的向量
    let my_ves = vec![1, 2, 3, 4, 5];

    // 安全的数组访问方式：使用索引操作符
    // 如果索引越界，程序会 panic
    println!("my_ves: {}", my_ves[0]);

    // 被注释掉的越界访问，会导致 panic
    // println!("my_ves: {}", my_ves[10]);

    // 安全的访问方式：使用 get() 方法返回 Option<T>
    // 如果索引越界，返回 None，不会 panic
    if let Some(value) = my_ves.get(11) {
        println!("value: {}", value);
    } else {
        println!("no value");
    }

    // unsafe 块：使用 get_unchecked() 方法
    // ⚠️ 危险：不进行边界检查，可能导致未定义行为
    unsafe {
        let value = my_ves.get_unchecked(11);
        println!("value: {}", value);
    }

    // 调用 unsafe 函数
    unsafe {
        my_fn();
    }
}

/// 演示 unsafe 函数的使用
///
/// # Safety
///
/// 此函数被标记为 unsafe，因为它：
/// 1. 使用了 get_unchecked() 方法进行未检查的数组访问
/// 2. 可能导致未定义行为（如内存访问错误）
/// 3. 调用者必须确保调用条件的安全性
///
/// 调用此函数时，调用者需要保证：
/// - 传入的向量有足够的大小
/// - 索引访问不会越界
unsafe fn my_fn() {
    // 创建新的向量实例
    let my_ves = vec![1, 2, 3, 4, 5];

    // ⚠️ 极度危险：访问超出向量边界的索引
    // 这会导致读取未初始化的内存，是典型的未定义行为
    let value = my_ves.get_unchecked(11);
    println!("unsafe function value: {}", value);
}

/*
🔗 相关学习资源：

📚 官方文档：
- Rust Book - Unsafe Rust: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
- Rustonomicon (The Rust Book of Unsafety): https://doc.rust-lang.org/nomicon/
- Unsafe Blocks Reference: https://doc.rust-lang.org/reference/unsafe-blocks.html

📖 中文资源：
- Rust语言圣经 - Unsafe Rust: https://course.rs/advance/unsafe.html
- Rust By Example - Unsafe: https://rustwiki.org/zh-CN/rust-by-example/unsafe.html
- Rust 中文文档 - unsafe关键字: https://www.rustwiki.org.cn/keyword/unsafe.html

⚡ 最佳实践指南：
- Rust Unsafe Guidelines: https://rust-lang.github.io/unsafe-code-guidelines/
- Writing Unsafe Rust: https://doc.rust-lang.org/nightly/nomicon/writing-unsafe-rust.html

🎯 关键概念：
- 边界检查 (Bounds Checking): Rust在运行时检查数组/向量访问是否在有效范围内
- 未定义行为 (Undefined Behavior): 程序行为无法预测，可能导致崩溃或安全漏洞
- 内存安全: Rust的核心特性，防止常见的内存错误如缓冲区溢出、悬垂指针等

⚠️ 重要提醒：
- unsafe 不等于不安全，而是告诉编译器"我知道我在做什么"
- 使用unsafe时必须确保手动维护Rust通常提供的安全保证
- 仅在绝对必要且了解所有风险时才使用unsafe代码
- 优先寻找安全的替代方案
*/
