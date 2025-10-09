/*
 * Rust unsafe 代码深入学习示例
 *
 * 本项目演示了 Rust 中 unsafe 代码的各种使用场景、危险性和最佳实践：
 * - 安全 vs 不安全的内存访问
 * - unsafe 函数和 unsafe 块的使用
 * - 边界检查的重要性
 * - 未定义行为的概念和危险
 *
 * 📚 官方文档链接：
 *
 * 🔰 基础概念
 * 1. Rust Book - Unsafe Rust:
 *    https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
 *
 * 2. Rustonomicon (不安全 Rust 圣经):
 *    https://doc.rust-lang.org/nomicon/
 *
 * 3. Unsafe 块参考:
 *    https://doc.rust-lang.org/reference/unsafe-blocks.html
 *
 * ⚙️ 高级概念
 * 4. 不安全代码指南:
 *    https://rust-lang.github.io/unsafe-code-guidelines/
 *
 * 5. 编写不安全 Rust:
 *    https://doc.rust-lang.org/nightly/nomicon/writing-unsafe-rust.html
 *
 * 6. 原始类型和裸指针:
 *    https://doc.rust-lang.org/std/primitive.pointer.html
 *
 * 🚀 中文资源
 * 7. Rust语言圣经 - Unsafe Rust:
 *    https://course.rs/advance/unsafe.html
 *
 * 8. Rust By Example - Unsafe:
 *    https://rustwiki.org/zh-CN/rust-by-example/unsafe.html
 *
 * ⚠️ 安全提醒
 * - unsafe 不等于不安全，而是告诉编译器"我知道我在做什么"
 * - 仅在绝对必要且了解所有风险时才使用 unsafe 代码
 * - 优先寻找安全的替代方案
 */

/// Rust unsafe 代码演示主函数
///
/// 本函数演示了 Rust 中安全与不安全内存访问的对比：
/// 1. 使用索引操作符的安全访问（带边界检查）
/// 2. 使用 get() 方法的安全访问（返回 Option）
/// 3. 使用 get_unchecked() 的不安全访问（无边界检查）
/// 4. unsafe 函数的调用
fn main() {
    println!("🦀 Rust unsafe 代码演示");
    println!("=======================");

    // 创建一个包含 5 个元素的向量
    // 向量在堆上分配内存，包含连续的整数值
    let my_ves = vec![1, 2, 3, 4, 5];
    println!("创建向量: {:?}", my_ves);

    // 方式 1: 安全的数组访问方式 - 使用索引操作符
    // Rust 会在运行时检查索引是否越界，如果越界会 panic
    println!("\n✅ 方式 1: 索引访问（安全，带边界检查）");
    println!("my_ves[0] = {}", my_ves[0]);
    println!("特点: 越界时程序会 panic，确保内存安全");

    // 被注释掉的越界访问示例
    // 取消注释会导致 panic
    // println!("my_ves[10] = {}", my_ves[10]); // 这会 panic!

    // 方式 2: 使用 get() 方法的安全访问
    // 返回 Option<T> 类型，优雅处理越界情况
    println!("\n✅ 方式 2: get() 方法访问（安全，返回 Option）");
    match my_ves.get(11) {
        Some(value) => println!("my_ves.get(11) = {}", value),
        None => println!("my_ves.get(11) = None（索引越界）"),
    }

    // 方式 3: unsafe 块中使用 get_unchecked() 方法
    // ⚠️ 危险：不进行边界检查，可能导致未定义行为
    println!("\n⚠️ 方式 3: get_unchecked() 访问（不安全，无边界检查）");
    println!("这是演示危险性的示例 - 在实际代码中应该避免！");
    unsafe {
        // 这个访问是未定义行为，因为索引 11 超出了向量范围 [0, 4]
        // 可能读取到垃圾数据，或者导致程序崩溃
        let value = my_ves.get_unchecked(11);
        println!("my_ves.get_unchecked(11) = {}", value);
        println!("警告: 这是未定义行为，结果不可预测！");
    }

    // 方式 4: 调用 unsafe 函数
    println!("\n⚠️ 方式 4: 调用 unsafe 函数");
    println!("调用包含危险操作的 unsafe 函数...");
    unsafe {
        my_fn();
    }

    // 安全性总结
    print_safety_summary();
}

/// 演示 unsafe 函数的使用和危险
///
/// 这个函数展示了 unsafe 函数的典型特征：
/// - 函数签名被标记为 unsafe
/// - 包含可能导致未定义行为的操作
/// - 需要调用者确保安全条件
///
/// # Safety
///
/// 调用此函数需要满足以下条件（虽然本示例中故意违反）：
/// - 确保向量有足够的元素
/// - 确保索引访问不会越界
/// - 确保内存访问是有效的
///
/// ⚠️ 重要警告：
/// 这个函数故意包含危险的代码来演示未定义行为的后果
/// 在实际代码中，这种写法是绝对错误的！
unsafe fn my_fn() {
    println!("进入了 unsafe 函数...");

    // 创建新的向量实例，包含 5 个元素
    let my_ves = vec![1, 2, 3, 4, 5];
    println!("函数内创建向量: {:?}", my_ves);
    println!("向量有效索引范围: 0 .. {}", my_ves.len());

    // ⚠️ 极度危险：访问超出向量边界的索引
    // 这会导致以下问题之一：
    // 1. 读取到未初始化的内存数据
    // 2. 读取到其他变量的数据
    // 3. 触发段错误导致程序崩溃
    // 4. 产生不可预测的结果
    println!("尝试访问索引 11（越界访问）...");

    let value = my_ves.get_unchecked(11);
    println!("unsafe function value: {}", value);
    println!("⚠️ 警告：刚才的操作是未定义行为！");
}

/// 打印安全性总结和建议
fn print_safety_summary() {
    println!("\n📊 安全性总结");
    println!("=============");

    println!("\n✅ 安全的访问方式:");
    println!("1. 使用索引操作符 [] - 自动边界检查，越界时 panic");
    println!("2. 使用 get() 方法 - 返回 Option<T>，优雅处理越界");
    println!("3. 使用迭代器 - 类型安全，编译时保证");

    println!("\n⚠️ 不安全的访问方式:");
    println!("1. 使用 get_unchecked() - 无边界检查，可能导致未定义行为");
    println!("2. 裸指针解引用 - 完全绕过 Rust 的安全检查");

    println!("\n💡 最佳实践:");
    println!("1. 优先使用安全的 API");
    println!("2. 只有在性能关键且确保安全时才使用 unsafe");
    println!("3. 为 unsafe 代码编写详细的文档和安全契约");
    println!("4. 尽可能将 unsafe 代码封装在安全的接口后面");

    println!("\n🔍 未定义行为的后果:");
    println!("1. 程序崩溃");
    println!("2. 数据损坏");
    println!("3. 安全漏洞");
    println!("4. 不可预测的行为");

    println!("\n📚 延伸学习:");
    println!("• Rustonomicon: https://doc.rust-lang.org/nomicon/");
    println!("• Unsafe 指南: https://rust-lang.github.io/unsafe-code-guidelines/");
}

/*
🔗 扩展学习资源：

📚 官方文档：
• Rust Book - Unsafe Rust: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
• Rustonomicon (不安全 Rust 圣经): https://doc.rust-lang.org/nomicon/
• Unsafe 块参考: https://doc.rust-lang.org/reference/unsafe-blocks.html
• 原始类型文档: https://doc.rust-lang.org/std/primitive.pointer.html

📖 中文资源：
• Rust语言圣经 - Unsafe Rust: https://course.rs/advance/unsafe.html
• Rust By Example - Unsafe: https://rustwiki.org/zh-CN/rust-by-example/unsafe.html
• Rust 中文社区 - unsafe 讨论: https://rust.cc/

⚡ 最佳实践指南：
• 不安全代码指南: https://rust-lang.github.io/unsafe-code-guidelines/
• 编写不安全 Rust: https://doc.rust-lang.org/nightly/nomicon/writing-unsafe-rust.html
• FFI 安全指南: https://rust-lang.github.io/unsafe-code-guidelines/ffi.html

🎯 关键概念解析：
• 边界检查 (Bounds Checking): Rust 在运行时检查数组/向量访问的有效性
• 未定义行为 (Undefined Behavior): 程序行为不可预测的严重问题
• 内存安全: Rust 防止缓冲区溢出、悬垂指针等内存错误的核心特性
• 安全契约 (Safety Contract): unsafe 函数要求调用者保证的条件
• 零成本抽象: Rust 在保证安全的同时不影响运行时性能

💻 实际应用场景：
• FFI (外部函数接口): 与 C/C++ 代码交互
• 高性能计算: 在确保安全的前提下优化性能瓶颈
• 底层系统编程: 直接操作硬件或内存
• 自定义数据结构: 实现编译器无法自动验证的安全性

⚠️ 安全警示：
• unsafe 不等于不安全，而是"我了解风险并承担后果"
• 使用 unsafe 时必须手动维护 Rust 的安全保证
• 优先寻找安全的替代方案，谨慎使用 unsafe
• 为 unsafe 代码编写充分的测试和文档
*/
