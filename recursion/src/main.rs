//!
//! # Rust 异步递归示例项目
//!
//! 本项目演示了 Rust 中异步递归函数的实现方法，以及相关的高级异步编程概念。
//!
//! ## 学习目标
//! - 理解异步递归函数的工作原理
//! - 掌握 `async-recursion` 库的使用
//! - 学习 `Pin<Box<dyn Future>>` 的使用方法
//! - 了解 Tokio 运行时的基本概念
//!
//! ## 重要概念
//!
//! ### 异步递归 (Async Recursion)
//! 在异步编程中，递归函数需要特殊的处理，因为普通的递归函数在编译时无法确定其大小。
//! `async-recursion` 库通过宏来自动处理这个限制。
//!
//! ### Future 和 Pin
//! - `Future`: Rust 异步编程的核心 trait，表示一个可能在未来完成的值
//! - `Pin`: 确保数据在内存中的位置不会改变，这对于自引用类型非常重要
//! - `Box<dyn Future>`: 动态分发 Future 类型，允许在运行时选择不同的异步操作
//!
//! ## 相关文档链接
//! - [Rust 异步编程指南](https://rust-lang.github.io/async-book/)
//! - [Tokio 官方文档](https://tokio.rs/tokio/tutorial)
//! - [async-recursion crate 文档](https://docs.rs/async-recursion/latest/async_recursion/)
//! - [Future trait 文档](https://doc.rust-lang.org/std/future/trait.Future.html)
//! - [Pin 类型文档](https://doc.rust-lang.org/std/pin/struct.Pin.html)

use async_recursion::*;
use std::pin::Pin;

// 导入 Pin 示例模块
mod pin_examples;
use pin_examples::*;

/// 使用异步递归实现的斐波那契数列
///
/// # 参数
/// * `n` - 要计算的斐波那契数列位置
///
/// # 返回值
/// 斐波那契数列第 n 项的值
///
/// # 示例
/// ```rust
/// let result = fibonacci(10).await;
/// println!("{}", result); // 输出 89
/// ```
///
/// # 注意事项
/// - 这是一个教学示例，实际应用中效率较低
/// - 大数值计算时建议使用动态规划或迭代方法
/// - 演示了 `async-recursion` 宏的基本用法
///
/// # 相关算法
/// - 时间复杂度: O(2^n) - 指数级，适合教学但不适合生产环境
/// - 空间复杂度: O(n) - 递归调用栈深度
///
/// # 学习资源
/// - [斐波那契数列维基百科](https://zh.wikipedia.org/wiki/斐波那契数列)
/// - [递归算法教程](https://www.runoob.com/data-structures/data-structures-tutorial-recursion.html)
#[async_recursion]
async fn fibonacci(n: u32) -> u32 {
    match n {
        // 基础情况：fibonacci(0) = 1
        0 => 1,
        // 基础情况：fibonacci(1) = 1
        1 => 1,
        // 递归情况：fibonacci(n) = fibonacci(n-1) + fibonacci(n-2)
        _ => fibonacci(n - 1).await + fibonacci(n - 2).await,
    }
}

/// 简单的异步函数 - 打印 "one"
///
/// 这是一个基础异步函数，用于演示异步函数的基本概念
async fn one() {
    println!("one");
}

/// 简单的异步函数 - 打印 "two"
///
/// 另一个基础异步函数，用于配合 one() 函数演示函数选择
async fn two() {
    println!("two");
}

/// 根据输入调用不同的异步函数
///
/// 这个函数演示了如何在运行时动态选择要执行的异步函数，
/// 并立即执行它。这是更常见的异步编程模式。
///
/// # 参数
/// * `n` - 选择要调用的函数 (1 或 2)
///
/// # 示例
/// ```rust
/// call_one_or_two(1).await; // 打印 "one"
/// call_one_or_two(2).await; // 打印 "two"
/// ```
async fn call_one_or_two(n: u32) {
    match n {
        1 => one().await,    // 直接调用并等待 one() 函数
        2 => two().await,    // 直接调用并等待 two() 函数
        _ => panic!("Invalid choice: must be 1 or 2"),
    }
}

/// 返回装箱 Future 的版本（仅用于演示，不推荐使用）
///
/// 这个函数展示如何返回 Pin<Box<dyn Future>> 类型，
/// 注意这个函数不是异步的，直接返回一个 Future。
///
/// # 重要说明
/// 这个函数返回的 Future 不会自动执行！调用者需要：
/// ```rust
/// let future = call_one_or_two_boxed(1);
/// tokio::pin!(future);
/// future.await; // 或者使用其他方式 poll Future
/// ```
fn call_one_or_two_boxed(n: u32) -> Pin<Box<dyn Future<Output = ()>>> {
    match n {
        1 => {
            // 将 one() 函数装箱并固定
            // Box::pin() 创建一个 Pin<Box<dyn Future>> 类型
            Box::pin(one())
        },
        2 => {
            // 将 two() 函数装箱并固定
            Box::pin(two())
        },
        _ => panic!("Invalid choice: must be 1 or 2"),
    }
}

/// 程序入口点 - 演示异步递归和 Future 操作
///
/// # 执行步骤
/// 1. 计算并打印斐波那契数列第 10 项
/// 2. 创建并执行一个简单的异步块
/// 3. 使用 tokio::pin! 宏固定 Future
/// 4. 演示动态函数选择
///
/// # tokio::main 宏说明
/// 这个宏会自动创建一个 Tokio 运行时，并在其中执行 async main 函数。
///
/// # tokio::pin! 宏说明
/// 将变量固定到栈上，确保它在内存中的位置不会改变。
/// 这对于自引用的 Future 类型是必需的。
#[tokio::main]
async fn main() {
    // 1. 演示异步递归：计算斐波那契数列
    // fibonacci(10) = 89，这是一个很好的测试用例
    println!("fibonacci(10): {}", fibonacci(10).await);

    // 2. 创建一个简单的异步块
    let future = async {
        println!("Hello world");
    };

    // 3. 使用 tokio::pin! 宏固定 Future
    // 这确保了 future 不会被移动，保证内存安全
    tokio::pin!(future);

    // 4. 执行被固定的 Future
    // 注意：这里使用 &mut future，因为 future 被 pin! 宏消耗了
    (&mut future).await;

    // 5. 演示动态函数选择
    println!("动态函数选择演示:");

    // 直接调用异步函数进行对比测试
    println!("直接调用 one():");
    one().await;

    println!("直接调用 two():");
    two().await;

    println!("通过动态选择调用 (推荐方式):");
    call_one_or_two(1).await; // 调用 one() 函数
    call_one_or_two(2).await; // 调用 two() 函数

    println!("演示装箱 Future 的使用 (高级用法):");
    println!("获取装箱 Future 1...");
    let boxed_future1 = call_one_or_two_boxed(1);
    tokio::pin!(boxed_future1);
    boxed_future1.await;

    println!("获取装箱 Future 2...");
    let boxed_future2 = call_one_or_two_boxed(2);
    tokio::pin!(boxed_future2);
    boxed_future2.await;

    println!("函数调用完成");

    // 6. 演示 Pin 类型的详细使用
    println!();
    println!("{}", "=".repeat(50));
    println!("📚 开始 Pin 类型详细演示");
    println!("{}", "=".repeat(50));
    run_all_pin_examples().await;
}
