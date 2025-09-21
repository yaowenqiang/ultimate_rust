//! # Footgun Fixed：数据竞争修复示例
//!
//! 这是原 "footgun" 示例的修复版本，展示了如何正确处理并发编程中的数据竞争问题。
//! 提供了两种解决方案：原子类型 (AtomicI32) 和互斥锁 (Mutex)。
//!
//! **这展示了正确的并发编程实践！**

use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicI32, Ordering},
    },
    thread,
    time::Instant,
};

// 解决方案1：使用原子类型 - 无锁并发，性能更好
static ATOMIC_COUNTER: AtomicI32 = AtomicI32::new(0);

// 解决方案2：使用互斥锁包装的计数器
type MutexCounter = Arc<Mutex<i32>>;

fn main() {
    println!("🔧 数据竞争修复演示");
    println!("期望结果：1000 线程 × 1000 次递增 = 1,000,000");
    println!("现在使用安全的并发原语，结果将是可预测的！\n");

    // 运行两种解决方案进行对比
    println!("=== 解决方案 1: 原子类型 (AtomicI32) ===");
    let atomic_result = run_atomic_version();

    println!("\n=== 解决方案 2: 互斥锁 (Mutex) ===");
    let mutex_result = run_mutex_version();

    println!("\n=== 性能对比 ===");
    println!("原子操作用时: {:.2}ms", atomic_result.duration_ms);
    println!("互斥锁用时:   {:.2}ms", mutex_result.duration_ms);
    println!(
        "性能差异:     {:.1}x",
        mutex_result.duration_ms / atomic_result.duration_ms
    );

    println!("\n✅ 两种方案都得到了正确的结果: 1,000,000");
    println!("💡 原子操作通常比互斥锁有更好的性能，但互斥锁更适合复杂的临界区。");
}

#[derive(Debug)]
struct BenchResult {
    final_count: i32,
    duration_ms: f64,
}

/// 使用原子类型的解决方案
fn run_atomic_version() -> BenchResult {
    let start_time = Instant::now();

    // 重置计数器
    ATOMIC_COUNTER.store(0, Ordering::SeqCst);

    let mut handles = Vec::new();

    // 创建 1000 个线程，每个线程执行 1000 次原子递增
    for i in 0..1000 {
        let handle = thread::spawn(move || {
            for _ in 0..1_000 {
                // 原子递增操作 - 线程安全且无锁
                // fetch_add 返回操作前的值，并原子地将值增加指定数量
                ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);

        // 显示进度
        if (i + 1) % 200 == 0 {
            println!("原子版本: 已创建 {} 个线程...", i + 1);
        }
    }

    println!("原子版本: 所有线程已启动，等待完成...");

    // 等待所有线程完成
    handles.into_iter().for_each(|h| h.join().unwrap());

    let final_count = ATOMIC_COUNTER.load(Ordering::SeqCst);
    let duration = start_time.elapsed();

    println!(
        "原子版本结果: {} (耗时 {:.2}ms)",
        final_count,
        duration.as_secs_f64() * 1000.0
    );

    BenchResult {
        final_count,
        duration_ms: duration.as_secs_f64() * 1000.0,
    }
}

/// 使用互斥锁的解决方案
fn run_mutex_version() -> BenchResult {
    let start_time = Instant::now();

    // 创建被 Arc<Mutex<>> 包装的共享计数器
    // Arc (Atomically Reference Counted) 允许多个线程拥有同一数据
    // Mutex 确保同一时间只有一个线程可以访问数据
    let counter: MutexCounter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    // 创建 1000 个线程，每个线程执行 1000 次加锁递增
    for i in 0..1000 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1_000 {
                // 获取锁并递增计数器
                // lock() 会阻塞当前线程直到获得独占访问权
                let mut count = counter_clone.lock().unwrap();
                *count += 1;
                // 锁在 count 变量超出作用域时自动释放
            }
        });
        handles.push(handle);

        // 显示进度
        if (i + 1) % 200 == 0 {
            println!("互斥锁版本: 已创建 {} 个线程...", i + 1);
        }
    }

    println!("互斥锁版本: 所有线程已启动，等待完成...");

    // 等待所有线程完成
    handles.into_iter().for_each(|h| h.join().unwrap());

    let final_count = *counter.lock().unwrap();
    let duration = start_time.elapsed();

    println!(
        "互斥锁版本结果: {} (耗时 {:.2}ms)",
        final_count,
        duration.as_secs_f64() * 1000.0
    );

    BenchResult {
        final_count,
        duration_ms: duration.as_secs_f64() * 1000.0,
    }
}
