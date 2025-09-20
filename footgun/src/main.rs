//! # Footgun 示例：数据竞争演示
//! 
//! 这是一个 "footgun" (容易出错的代码) 示例，用于演示 Rust 中不安全的并发编程。
//! 这段代码故意展示了数据竞争的危险性和不确定行为。
//! 
//! **警告：这是反面教材，展示了什么是不应该做的！**

use std::{ptr::addr_of, thread};

// 全局可变静态变量 - 这是危险的根源！
// 在多线程环境中，没有同步机制的全局可变状态会导致数据竞争
static mut COUNTER: i32 = 0;

fn main() {
    println!("开始数据竞争演示...");
    println!("期望结果：1000 线程 × 1000 次递增 = 1,000,000");
    println!("实际结果将因数据竞争而变得不可预测\n");

    // 存储所有线程句柄的容器
    let mut handles = Vec::new();
    
    // 创建 1000 个线程，每个线程都会尝试修改同一个全局变量
    for i in 0..1000 {
        let handle = thread::spawn(move || {
            // 每个线程执行 1000 次递增操作
            for _ in 0..1_000 {
                unsafe {
                    // 危险操作！多个线程同时修改全局变量而没有同步
                    // 这会导致：
                    // 1. 数据竞争 (data race)
                    // 2. 不确定的结果
                    // 3. 可能的内存损坏
                    COUNTER += 1;
                }
            }
        });
        handles.push(handle);
        
        // 每创建 100 个线程就显示一次进度
        if (i + 1) % 100 == 0 {
            println!("已创建 {} 个线程...", i + 1);
        }
    }

    println!("所有线程已启动，等待完成...");
    
    // 等待所有线程完成
    // join() 确保主线程等待所有子线程执行完毕
    handles.into_iter().for_each(|h| h.join().unwrap());

    // 读取最终结果
    // 使用 addr_of! 宏避免创建对可变静态变量的共享引用
    // 这是 Rust 2024 版本的要求
    unsafe {
        let final_count = *addr_of!(COUNTER);
        println!("\n=== 结果分析 ===");
        println!("最终计数值: {}", final_count);
        println!("期望值: 1,000,000");
        println!("丢失的递增: {}", 1_000_000 - final_count);
        println!("丢失率: {:.2}%", (1_000_000 - final_count) as f64 / 10_000.0);
        
        if final_count < 1_000_000 {
            println!("\n⚠️  数据竞争导致了递增操作的丢失！");
            println!("这就是为什么需要使用 Mutex、Atomic 等同步原语的原因。");
        } else {
            println!("\n🎲 这次运行很幸运，但结果仍然是不可预测的！");
        }
    }

    println!("\n💡 正确的做法应该使用:");
    println!("   - std::sync::Mutex<i32> 用于互斥访问");
    println!("   - std::sync::atomic::AtomicI32 用于原子操作");
    println!("   - 或其他适当的同步原语");
}
