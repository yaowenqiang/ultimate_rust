//! # Footgun Manual - 数据竞争修复成功示例
//! 
//! 这是对原始 "footgun" 示例的完美修复，使用原子操作彻底消除了数据竞争。
//! 
//! ## 修复方案
//! 使用 `std::sync::atomic::AtomicI32` 替代不安全的 `static mut`，
//! 通过原子操作确保多线程环境下的内存安全。
//! 
//! ## 相关文档
//! - [Rust Book - 并发章节](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
//! - [AtomicI32 官方文档](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicI32.html)
//! - [原子内存排序](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html)
//! - [线程文档](https://doc.rust-lang.org/std/thread/index.html)

use std::{
    sync::atomic::{AtomicI32, Ordering},
    thread,
    time::Instant,
};

/// 全局原子计数器 - 线程安全的共享状态
/// 
/// 使用 AtomicI32 替代 `static mut i32` 确保:
/// - 无数据竞争 (data race free)
/// - 内存安全 (memory safe) 
/// - 无锁并发 (lock-free concurrency)
/// 
/// 文档: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicI32.html
static COUNTER: AtomicI32 = AtomicI32::new(0);

/// 线程计数常量 - 要创建的线程数量
const NUM_THREADS: usize = 1_000;

/// 每个线程的递增次数
const INCREMENTS_PER_THREAD: usize = 1_000;

/// 期望的最终计数结果
const EXPECTED_TOTAL: i32 = (NUM_THREADS * INCREMENTS_PER_THREAD) as i32;

fn main() {
    println!("🔧 Footgun 修复成功 - 原子操作方案");
    println!("期望结果: {} 线程 × {} 次递增 = {}", 
             NUM_THREADS, INCREMENTS_PER_THREAD, EXPECTED_TOTAL);
    println!("使用 AtomicI32 确保线程安全和确定性结果\n");

    let start_time = Instant::now();
    
    // 存储所有线程句柄的向量
    // 使用 Vec::with_capacity 预分配容量以提高性能
    let mut handles = Vec::with_capacity(NUM_THREADS);

    println!("📊 创建 {} 个并发线程...", NUM_THREADS);
    
    // 创建指定数量的线程，每个线程执行原子递增操作
    for thread_id in 0..NUM_THREADS {
        // thread::spawn 创建新的操作系统线程
        // move 关键字将 thread_id 的所有权转移到闭包中
        // 文档: https://doc.rust-lang.org/std/thread/fn.spawn.html
        let handle = thread::spawn(move || {
            // 每个线程执行指定次数的原子递增操作
            for _ in 0..INCREMENTS_PER_THREAD {
                // fetch_add: 原子地将值加1并返回操作前的值
                // Ordering::Relaxed: 最宽松的内存排序，性能最优
                // 对于简单计数器来说足够安全
                // 文档: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicI32.html#method.fetch_add
                COUNTER.fetch_add(1, Ordering::Relaxed);
            }
        });
        
        handles.push(handle);
        
        // 每创建 100 个线程显示一次进度
        if (thread_id + 1) % 100 == 0 {
            println!("  已创建 {} / {} 线程", thread_id + 1, NUM_THREADS);
        }
    }

    println!("⏳ 等待所有线程完成执行...");
    
    // 等待所有线程完成执行
    // join() 确保主线程等待子线程完成，防止程序过早退出
    // into_iter() 获取 handles 的所有权，避免不必要的借用
    // unwrap() 在这里是安全的，因为我们的线程不会 panic
    // 文档: https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#method.join
    handles.into_iter().for_each(|handle| {
        handle.join().expect("线程执行失败");
    });
    
    let elapsed = start_time.elapsed();

    // 原子地读取最终计数值
    // load() 方法原子地读取当前值
    // 使用 Relaxed 排序与写操作保持一致
    let final_count = COUNTER.load(Ordering::Relaxed);
    
    println!("\n=== 执行结果分析 ===");
    println!("🎯 最终计数: {}", final_count);
    println!("🎯 期望计数: {}", EXPECTED_TOTAL);
    println!("⏱️  执行耗时: {:.2}ms", elapsed.as_secs_f64() * 1000.0);
    
    // 验证修复是否成功
    if final_count == EXPECTED_TOTAL {
        println!("\n🎉 修复成功！");
        println!("✅ 结果完全正确且确定性");
        println!("✅ 无数据竞争，完全线程安全");
        println!("✅ 原子操作确保内存一致性");
        
        // 计算性能指标
        let operations_per_sec = EXPECTED_TOTAL as f64 / elapsed.as_secs_f64();
        println!("🚀 性能: {:.0} 次原子操作/秒", operations_per_sec);
    } else {
        // 这种情况在正确实现中不应该发生
        println!("\n❌ 意外错误: 计数不匹配");
        println!("差异: {}", EXPECTED_TOTAL - final_count);
    }
    
    println!("\n📚 技术要点:");
    println!("   🔹 AtomicI32: 无锁原子操作，性能优异");
    println!("   🔹 Relaxed 排序: 适合简单计数器的最优选择");
    println!("   🔹 fetch_add(): 原子递增，返回操作前的值");
    println!("   🔹 load(): 原子读取当前值");
    
    println!("\n🔗 参考文档:");
    println!("   📖 https://doc.rust-lang.org/std/sync/atomic/struct.AtomicI32.html");
    println!("   📖 https://doc.rust-lang.org/book/ch16-00-concurrency.html");
    println!("   📖 https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html");
    println!("   📖 https://doc.rust-lang.org/std/thread/index.html");
}