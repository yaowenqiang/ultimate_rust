//!
//! # Rust Pin 类型详细示例
//!
//! 本文件提供了 Pin 类型的全面示例，包括：
//! - Pin 的基本概念和使用方法
//! - 为什么要使用 Pin 的场景对比
//! - Pin 的高级用法和最佳实践
//!
//! ## 什么是 Pin？
//!
//! Pin 是 Rust 中的一个类型，用于"固定"数据在内存中的位置，确保它不会被移动。
//! 这对于自引用类型（结构体包含指向自身的引用）特别重要。
//!
//! ## Pin 的核心概念
//!
//! 1. **Pin<P>**: 包装一个指针 P，确保指向的数据不会被移动
//! 2. **Unpin**: 标记 trait，表示类型可以安全地移动
//! 3. **!Unpin**: 类型不能安全地移动，需要 Pin 来保证安全
//!
//! ## 相关文档链接
//! - [Pin 模块文档](https://doc.rust-lang.org/std/pin/index.html)
//! - [Pin 结构体文档](https://doc.rust-lang.org/std/pin/struct.Pin.html)
//! - [Pinning 机制详解](https://rust-lang.github.io/async-book/04_pinning/01_chapter.html)
//! - [Unpin trait 文档](https://doc.rust-lang.org/std/marker/trait.Unpin.html)

use std::marker::PhantomPinned;
use std::pin::Pin;

/// ============================================================================
/// 1. Pin 基础概念示例
/// ============================================================================

/// 演示 Pin 的基本创建和使用
pub fn basic_pin_examples() {
    println!("=== Pin 基础示例 ===");

    // 1.1 在栈上创建 Pin
    let mut value = 42;
    println!("原始值: {}", value);

    let pinned_ref = Pin::new(&mut value);
    println!("通过 Pin 访问: {}", pinned_ref);

    // 1.2 在堆上创建 Pin (Box)
    let boxed_value = Box::new(42);
    let pinned_box = Pin::new(boxed_value);

    println!("装箱值: {}", pinned_box);

    // 1.3 直接创建 Pin<Box<>>
    let pinned_box_direct: Pin<Box<i32>> = Box::pin(42);
    println!("直接创建的 Pin<Box<>>: {}", pinned_box_direct);

    println!();
}

/// ============================================================================
/// 2. 自引用结构体示例
/// ============================================================================

/// 一个自引用结构体（简化版本，实际中需要更复杂的实现）
///
/// 注意：这个示例是为了教学目的，实际的自引用结构体通常需要使用 unsafe 代码
/// 或者专门的库如 `ouroboros`、`pin-project` 等
#[derive(Debug)]
pub struct SelfRefStruct {
    value: String,
    // 在实际实现中，这里会有一个指向自身的引用
    // pointer: *const str,
}

impl SelfRefStruct {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
            // pointer: null,
        }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}

/// 演示为什么需要 Pin
pub fn self_reference_example() {
    println!("=== 自引用结构体示例 ===");

    let data = SelfRefStruct::new("Hello");
    println!("创建的结构体: {:?}", data);
    println!("获取值: {}", data.get_value());

    // 在真实场景中，如果我们移动了 data，任何自引用都会变成悬垂指针
    // Pin 确保这种情况不会发生

    println!();
}

/// ============================================================================
/// 3. 不可移动类型示例
/// ============================================================================

/// 一个标记为 !Unpin 的类型，表示它不能安全地移动
#[derive(Debug)]
pub struct ImmovableStruct {
    _data: String,
    _pinned: PhantomPinned, // 这个字段使整个类型变为 !Unpin
}

impl ImmovableStruct {
    pub fn new(data: &str) -> Self {
        Self {
            _data: data.to_string(),
            _pinned: PhantomPinned,
        }
    }

    // 安全地访问数据的方法
    pub fn get_data(self: Pin<&mut Self>) -> &str {
        // 在实际应用中，这里需要更复杂的 unsafe 代码
        // 为了安全起见，我们简化实现
        "Immovable data"
    }
}

/// 演示不可移动类型的使用
pub fn immovable_example() {
    println!("=== 不可移动类型示例 ===");

    // 创建不可移动类型
    let immovable = ImmovableStruct::new("important data");

    // 必须使用 Pin 来处理不可移动类型
    let mut pinned_immovable = Box::pin(immovable);

    println!("不可移动类型数据: {}", pinned_immovable.as_mut().get_data());

    println!();
}

/// ============================================================================
/// 4. Pin 在异步编程中的应用
/// ============================================================================

/// 异步函数返回的自引用类型示例
pub async fn async_pin_example() {
    println!("=== 异步编程中的 Pin 示例 ===");

    // 在异步编程中，Future 经常需要自引用
    // 编译器会自动处理大部分 Pin 相关的事情

    let future = async {
        println!("这是一个异步 Future");
        "异步操作完成"
    };

    // 使用 Box::pin 将 Future 固定
    let pinned_future = Box::pin(future);
    let result = pinned_future.await;

    println!("Future 结果: {}", result);
    println!();
}

/// ============================================================================
/// 5. 对比示例：为什么需要 Pin
/// ============================================================================

/// 场景1：没有 Pin 的危险情况（概念演示）
pub fn dangerous_without_pin() {
    println!("=== 危险场景：没有 Pin 的情况 ===");

    // 模拟一个自引用结构体（概念演示）
    struct UnsafeSelfRef {
        data: String,
        // 在 unsafe 代码中，这里可能会有自引用指针
    }

    impl UnsafeSelfRef {
        fn new(data: &str) -> Self {
            Self {
                data: data.to_string(),
            }
        }

        fn get_data(&self) -> &str {
            &self.data
        }
    }

    let obj = UnsafeSelfRef::new("original");

    // 如果我们移动了这个对象，任何自引用都会失效
    let moved_obj = obj; // 移动发生

    println!("移动后的对象数据: {}", moved_obj.get_data());
    // 在真实场景中，如果 obj 有自引用，这里就会是悬垂指针！

    println!("注意：这个演示简化了问题，实际情况更复杂\n");
}

/// 场景2：使用 Pin 的安全情况
pub fn safe_with_pin() {
    println!("=== 安全场景：使用 Pin 的情况 ===");

    // 使用 Pin 确保对象不会被移动
    let data = String::from("safe data");
    let pinned_data = Pin::new(&data);

    // pinned_data 不能被移动，保证所有引用都有效
    println!("Pin 保护的数据: {}", pinned_data);

    // 任何尝试移动 pinned_data 的操作都会被编译器阻止
    // 这保证了内存安全

    println!("Pin 保证了内存安全\n");
}

/// ============================================================================
/// 6. Pin 与不同指针类型的组合
/// ============================================================================

/// 演示 Pin 与各种指针类型的组合使用
pub fn pin_pointer_combinations() {
    println!("=== Pin 与指针类型组合 ===");

    // 6.1 Pin<&T> - 不可变引用
    let value = 42;
    let pin_ref: Pin<&i32> = Pin::new(&value);
    println!("Pin<&i32>: {}", pin_ref);

    // 6.2 Pin<&mut T> - 可变引用
    let mut value = 42;
    let pin_mut_ref: Pin<&mut i32> = Pin::new(&mut value);
    println!("Pin<&mut i32> (修改前): {}", pin_mut_ref);

    // 注意：通过 Pin<&mut T> 修改数据需要特殊方法
    // 为了简化，我们只演示读取

    // 6.3 Pin<Box<T>> - 堆分配
    let pin_box: Pin<Box<i32>> = Box::pin(42);
    println!("Pin<Box<i32>>: {}", pin_box);

    // 6.4 Pin<Rc<T>> - 共享引用（需要 std::rc::Rc）
    // 注意：Rc 在 async 环境中不是 Send，所以这里只是概念演示
    println!("Pin<Rc<i32>> - 共享引用（在异步环境中有限制）");

    println!();
}

/// ============================================================================
/// 7. Pin 的实际应用场景
/// ============================================================================

/// Future 实现中的 Pin 使用示例

/// 演示 Future 中的 Pin 使用（简化版）
pub async fn custom_future_example() {
    println!("=== Future 中的 Pin 使用演示 ===");

    // 在实际的异步编程中，Pin 会被自动处理
    // 这里演示常见的 Future 操作

    println!("1. 创建并执行简单的 Future");
    let future1 = async {
        println!("   Future 1 执行中...");
        "结果1"
    };
    let result1 = future1.await;
    println!("   Future 1 结果: {}", result1);

    println!("\n2. 使用 Box::pin 处理复杂的 Future");
    let future2 = Box::pin(async {
        println!("   复杂 Future 执行中...");
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        "结果2"
    });
    let result2 = future2.await;
    println!("   复杂 Future 结果: {}", result2);

    println!("\n3. Pin 在 Future 链中的作用");
    let result3 = async {
        println!("   链式 Future 第一步");
        let intermediate = "中间值".to_string();
        async move {
            println!("   链式 Future 第二步: {}", intermediate);
            "最终结果"
        }
        .await
    }
    .await;
    println!("   链式 Future 结果: {}", result3);

    println!();
}

/// ============================================================================
/// 8. Pin 的最佳实践和注意事项
/// ============================================================================

/// Pin 使用的最佳实践示例
pub fn pin_best_practices() {
    println!("=== Pin 最佳实践 ===");

    // 8.1 优先使用 Box::pin() 创建 Pin<Box<>>
    let future = async { "hello" };
    let _pinned_future = Box::pin(future);

    println!("使用 Box::pin() 创建的 Future: Pin<Box<Future>>");

    // 8.2 对于栈上数据，使用 Pin::new()
    let mut value = 42;
    let pinned_value = Pin::new(&mut value);

    println!("使用 Pin::new() 创建的栈数据: {}", pinned_value);

    // 8.3 大多数类型实现了 Unpin，不需要特殊处理
    let normal_type = String::from("normal");
    println!("普通类型（Unpin）: {}", normal_type);

    // 8.4 只有自引用或特殊类型才需要考虑 Pin
    println!("大多数情况下，你不需要手动处理 Pin");
    println!("编译器会在异步编程中自动处理大部分情况\n");
}

/// ============================================================================
/// 9. Pin 相关的常见错误和解决方案
/// ============================================================================

/// 常见错误示例（编译时会被捕获）
pub fn common_pin_mistakes() {
    println!("=== Pin 常见错误和解决方案 ===");

    println!("1. 尝试移动被 Pin 的数据");
    println!("   // 这会编译错误！");
    println!("   let pinned = Pin::new(&mut value);");
    println!("   let moved = *pinned; // 错误！");

    println!("\n2. 在不需要 Pin 的情况下使用 Pin");
    println!("   // 这不是错误，但通常不必要");
    println!("   let value = 42;");
    println!("   let pinned = Pin::new(&value); // 可以，但没必要");

    println!("\n3. 忘记对 Future 使用 Pin");
    println!("   // 在手动实现 Future 时容易犯的错误");
    println!("   impl Future for MyType {{");
    println!("       fn poll(mut self: Pin<&mut Self>, ...) {{ ... }}");
    println!("       // 错误：不应该获取 mut self 的所有权");
    println!("   }}");

    println!("\n解决方案：");
    println!("- 使用 Box::pin() 处理需要 Pin 的类型");
    println!("- 使用 Pin::new() 处理栈上数据");
    println!("- 在自定义 Future 中正确使用 self: Pin<&mut Self>");
    println!("- 大多数情况下让编译器自动处理 Pin\n");
}

/// ============================================================================
/// 10. 性能考虑
/// ============================================================================

/// Pin 的性能影响分析
pub fn pin_performance_considerations() {
    println!("=== Pin 性能考虑 ===");

    // 10.1 Pin 本身没有运行时开销
    println!("1. Pin 是零成本抽象");
    println!("   - Pin 本身不包含额外数据");
    println!("   - 运行时性能与原始指针相同");

    // 10.2 Box::pin() 的开销
    println!("\n2. Box::pin() 的开销");
    println!("   - 一次堆分配");
    println!("   - 与普通的 Box<T> 相同");

    // 10.3 编译时检查
    println!("\n3. 编译时检查");
    println!("   - Pin 的安全性检查在编译时进行");
    println!("   - 运行时无额外检查开销");

    println!("\n总结：Pin 是高效的内存安全工具，零运行时开销\n");
}

/// ============================================================================
/// 主演示函数
/// ============================================================================

/// 运行所有 Pin 示例
pub async fn run_all_pin_examples() {
    println!("🎯 Rust Pin 类型完整演示\n");

    basic_pin_examples();
    self_reference_example();
    immovable_example();
    async_pin_example().await;

    dangerous_without_pin();
    safe_with_pin();

    pin_pointer_combinations();
    custom_future_example().await;

    pin_best_practices();
    common_pin_mistakes();
    pin_performance_considerations();

    println!("🎉 Pin 示例演示完成！");
}
