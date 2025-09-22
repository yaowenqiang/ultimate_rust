//! hello_tokio 性能对比示例
//! 
//! 本示例通过模拟 I/O 密集型任务，清晰地展示 Tokio 异步编程相比同步编程的性能优势：
//! 
//! **对比场景：**
//! 1. **同步版本**: 模拟同步网络请求，任务串行执行
//! 2. **异步版本**: 使用 Tokio 并发处理多个异步任务
//! 3. **混合负载**: 同时处理 I/O 和 CPU 任务，展示协作式调度优势
//! 
//! **性能测试场景：**
//! - 模拟 10 个网络请求，每个耗时 100ms
//! - 同步方式：总耗时约 1000ms（串行执行）
//! - 异步方式：总耗时约 100ms（并发执行）
//! - 性能提升：约 10 倍
//!
//! **相关文档：**
//! - Tokio 性能指南: https://tokio.rs/tokio/topics/performance
//! - 异步编程概念: https://rust-lang.github.io/async-book/
//! - tokio::time 模块: https://docs.rs/tokio/latest/tokio/time/index.html
//! - tokio::spawn 并发: https://docs.rs/tokio/latest/tokio/fn.spawn.html
//! - futures::join_all: https://docs.rs/futures/latest/futures/future/fn.join_all.html

use std::time::{Duration, Instant};
use tokio::time::sleep;

/// 模拟一个同步的网络请求或 I/O 操作
/// 
/// 在真实场景中，这可能是：
/// - 数据库查询
/// - HTTP 请求  
/// - 文件读写
/// - 网络 Socket 操作
fn simulate_sync_io_task(task_id: u32, delay_ms: u64) -> String {
    let start = Instant::now();
    
    // 使用 std::thread::sleep 模拟阻塞式 I/O
    // 注意：这会真正阻塞当前线程
    std::thread::sleep(Duration::from_millis(delay_ms));
    
    let elapsed = start.elapsed();
    let result = format!("Task {}: completed in {:?}", task_id, elapsed);
    println!("{}", result);
    result
}

/// 模拟一个异步的网络请求或 I/O 操作
/// 
/// 异步版本的关键区别：
/// - 使用 `tokio::time::sleep` 而不是 `std::thread::sleep`
/// - `tokio::time::sleep` 不会阻塞线程，而是让出执行权给其他任务
/// - 允许运行时在等待期间调度其他就绪任务
/// 
/// 文档：https://docs.rs/tokio/latest/tokio/time/fn.sleep.html
async fn simulate_async_io_task(task_id: u32, delay_ms: u64) -> String {
    let start = Instant::now();
    
    // 使用 tokio::time::sleep 模拟非阻塞式 I/O
    // 这不会阻塞线程，允许其他任务并发执行
    sleep(Duration::from_millis(delay_ms)).await;
    
    let elapsed = start.elapsed();
    let result = format!("Async Task {}: completed in {:?}", task_id, elapsed);
    println!("{}", result);
    result
}

/// CPU 密集型任务，用于演示协作式调度
/// 
/// 在长时间计算中插入 `yield_now()` 调用，
/// 避免独占线程导致其他任务饥饿
async fn cpu_intensive_task(task_id: u32, iterations: u32) -> u64 {
    let start = Instant::now();
    let mut sum = 0u64;
    
    for i in 0..iterations {
        // 简单的计算操作
        sum += (i as u64).wrapping_mul(i as u64);
        
        // 每1000次迭代主动让出一次，允许其他任务运行
        // 这展示了 Tokio 的协作式调度特性
        if i % 1000 == 0 {
            tokio::task::yield_now().await;
        }
    }
    
    let elapsed = start.elapsed();
    println!("CPU Task {}: processed {} iterations in {:?}, sum={}", 
             task_id, iterations, elapsed, sum);
    sum
}

/// 执行同步版本的性能测试
/// 
/// 所有任务串行执行，总时间是各任务时间之和
fn benchmark_sync_version(num_tasks: u32, delay_ms: u64) -> Duration {
    println!("🔄 执行同步版本 ({} 个任务，各耗时 {}ms)", num_tasks, delay_ms);
    let start = Instant::now();
    
    let mut results = Vec::new();
    for i in 1..=num_tasks {
        let result = simulate_sync_io_task(i, delay_ms);
        results.push(result);
    }
    
    let total_elapsed = start.elapsed();
    println!("📊 同步版本总耗时: {:?}", total_elapsed);
    println!("📈 平均每任务: {:?}", total_elapsed / num_tasks);
    println!();
    
    total_elapsed
}

/// 执行异步版本的性能测试（使用 tokio::spawn 并发执行）
/// 
/// 所有任务并发执行，总时间约等于单个任务时间
/// 
/// 使用 `tokio::spawn` 的优势：
/// - 每个任务作为独立的异步任务被调度
/// - 可以充分利用运行时的并发能力
/// - 在多线程运行时下，任务可能在不同线程上并行执行
async fn benchmark_async_spawn_version(num_tasks: u32, delay_ms: u64) -> Duration {
    println!("🚀 执行异步版本 - spawn 并发 ({} 个任务，各耗时 {}ms)", num_tasks, delay_ms);
    let start = Instant::now();
    
    // 创建多个并发任务
    let mut handles = Vec::new();
    for i in 1..=num_tasks {
        let handle = tokio::spawn(simulate_async_io_task(i, delay_ms));
        handles.push(handle);
    }
    
    // 等待所有任务完成
    let mut results = Vec::new();
    for handle in handles {
        match handle.await {
            Ok(result) => results.push(result),
            Err(e) => eprintln!("任务执行失败: {}", e),
        }
    }
    
    let total_elapsed = start.elapsed();
    println!("📊 异步 spawn 版本总耗时: {:?}", total_elapsed);
    println!("📈 平均每任务: {:?}", total_elapsed / num_tasks);
    println!();
    
    total_elapsed
}

/// 执行异步版本的性能测试（使用 futures::join_all 并发等待）
/// 
/// 另一种并发模式：不创建独立任务，而是并发地推进多个 Future
/// 
/// `join_all` vs `spawn` 的区别：
/// - `join_all`: 在当前任务上下文中并发推进所有 Future
/// - `spawn`: 为每个 Future 创建独立的任务，交给运行时调度
/// 
/// 文档：https://docs.rs/futures/latest/futures/future/fn.join_all.html
async fn benchmark_async_join_version(num_tasks: u32, delay_ms: u64) -> Duration {
    use futures::future::join_all;
    
    println!("⚡ 执行异步版本 - join_all 并发 ({} 个任务，各耗时 {}ms)", num_tasks, delay_ms);
    let start = Instant::now();
    
    // 创建多个 Future，但不立即执行
    let futures: Vec<_> = (1..=num_tasks)
        .map(|i| simulate_async_io_task(i, delay_ms))
        .collect();
    
    // 并发等待所有 Future 完成
    let results = join_all(futures).await;
    
    let total_elapsed = start.elapsed();
    println!("📊 异步 join_all 版本总耗时: {:?}", total_elapsed);
    println!("📈 平均每任务: {:?}", total_elapsed / num_tasks);
    println!("✅ 完成 {} 个任务", results.len());
    println!();
    
    total_elapsed
}

/// 混合负载测试：同时运行 I/O 和 CPU 任务
/// 
/// 展示 Tokio 协作式调度的优势：
/// - I/O 任务在等待时让出执行权
/// - CPU 任务通过 yield_now() 主动让出
/// - 运行时智能地在任务间切换，提高整体吞吐量
async fn benchmark_mixed_workload() -> Duration {
    println!("🔄 执行混合负载测试 (I/O + CPU 任务)");
    let start = Instant::now();
    
    // 使用 tokio::join! 并发执行不同类型的任务
    let (io_results, cpu_results) = tokio::join!(
        // I/O 密集型任务组
        async {
            let futures = (1..=5)
                .map(|i| simulate_async_io_task(i, 50))
                .collect::<Vec<_>>();
            futures::future::join_all(futures).await
        },
        // CPU 密集型任务组
        async {
            let mut handles = Vec::new();
            for i in 1..=3 {
                handles.push(tokio::spawn(cpu_intensive_task(i, 10000)));
            }
            let mut results = Vec::new();
            for handle in handles {
                results.push(handle.await.unwrap());
            }
            results
        }
    );
    
    let total_elapsed = start.elapsed();
    println!("📊 混合负载总耗时: {:?}", total_elapsed);
    println!("✅ I/O 任务完成: {}, CPU 任务完成: {}", 
             io_results.len(), cpu_results.len());
    println!();
    
    total_elapsed
}

/// 运行完整的性能对比测试套件
async fn run_performance_benchmarks() {
    println!("🎯 === Tokio 异步性能对比测试 ===\n");
    
    let num_tasks = 8;
    let delay_per_task = 100; // 100ms per task
    
    // 1. 同步版本基准测试
    let sync_time = benchmark_sync_version(num_tasks, delay_per_task);
    
    // 短暂休息，让输出更清晰
    sleep(Duration::from_millis(200)).await;
    
    // 2. 异步版本测试（spawn 方式）
    let async_spawn_time = benchmark_async_spawn_version(num_tasks, delay_per_task).await;
    
    sleep(Duration::from_millis(200)).await;
    
    // 3. 异步版本测试（join_all 方式）  
    let async_join_time = benchmark_async_join_version(num_tasks, delay_per_task).await;
    
    sleep(Duration::from_millis(200)).await;
    
    // 4. 混合负载测试
    let mixed_time = benchmark_mixed_workload().await;
    
    // 性能对比总结
    println!("📈 === 性能对比总结 ===");
    println!("同步版本耗时:           {:>8?}", sync_time);
    println!("异步版本 (spawn):       {:>8?}", async_spawn_time);
    println!("异步版本 (join_all):    {:>8?}", async_join_time);
    println!("混合负载:               {:>8?}", mixed_time);
    println!();
    
    // 计算性能提升比率
    let spawn_speedup = sync_time.as_secs_f64() / async_spawn_time.as_secs_f64();
    let join_speedup = sync_time.as_secs_f64() / async_join_time.as_secs_f64();
    
    println!("🚀 === 性能提升分析 ===");
    println!("Spawn 方式性能提升:     {:.1}x", spawn_speedup);
    println!("Join_all 方式性能提升:  {:.1}x", join_speedup);
    println!();
    
    println!("💡 === 关键洞察 ===");
    println!("• 同步方式: 任务串行执行，总时间 = 各任务时间之和");
    println!("• 异步方式: 任务并发执行，总时间 ≈ 单个任务时间");
    println!("• I/O 密集型场景下，异步编程可带来数量级的性能提升");
    println!("• 协作式调度确保了混合负载下的公平执行");
    println!();
    
    println!("📚 进一步学习:");
    println!("• Tokio 性能指南: https://tokio.rs/tokio/topics/performance");
    println!("• 异步编程最佳实践: https://rust-lang.github.io/async-book/");
    println!("• 选择合适的并发策略: spawn vs join vs select");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // 运行完整的性能对比测试
    run_performance_benchmarks().await;
    
    println!("✨ 性能测试完成!");
}