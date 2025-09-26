// Tokio Select 演示项目
// 展示如何使用 tokio::select! 宏来处理多个异步操作的竞争执行

use std::time::Duration;
// tokio::select! 宏用于等待多个异步操作中的任意一个完成
// 官方文档: https://docs.rs/tokio/latest/tokio/macro.select.html
use tokio::select;
// Tokio 提供的异步通道类型
// mpsc: 多生产者单消费者通道，broadcast: 广播通道
// 官方文档: https://docs.rs/tokio/latest/tokio/sync/index.html
use tokio::sync::{broadcast, mpsc};

/// 模拟一个耗时 2 秒的异步工作
/// 使用 tokio::time::sleep 进行非阻塞的异步等待
/// 文档: https://docs.rs/tokio/latest/tokio/time/fn.sleep.html
async fn do_work() {
    tokio::time::sleep(Duration::from_secs(2)).await;
}

/// 接收器函数：同时监听两种不同类型的通道
/// 
/// # 参数
/// * `rx` - MPSC 通道的接收端，用于点对点通信
/// * `broadcast_rx` - 广播通道的接收端，用于一对多通信
/// 
/// # 功能
/// 使用 tokio::select! 宏同时等待两个通道的消息
/// 哪个通道先有消息就先处理哪个，实现了非阻塞的多路复用
async fn receiver(mut rx: mpsc::Receiver<u32>, mut broadcast_rx: broadcast::Receiver<u32>) {
    loop {
        // tokio::select! 宏的核心用法
        // 同时等待多个异步操作，哪个先完成就执行对应的分支
        // 这是 Rust 异步编程中处理并发的重要工具
        tokio::select! {
            // 等待 MPSC 通道接收消息
            // Some(n) 模式匹配：如果接收到消息则执行此分支
            // None 表示通道已关闭，此时不会匹配这个分支
            Some(n) = rx.recv() => {
                println!("received message {} on the mpsc channel", n);
            },
            // 等待广播通道接收消息
            // Ok(n) 模式匹配：如果成功接收到消息则执行此分支
            // Err 表示接收失败（如通道关闭或滞后），此时不会匹配这个分支
            Ok(n) = broadcast_rx.recv() => {
                println!("received message {} on the broadcast channel", n);
            }
        }
    }
}

/// 超时函数：等待指定的秒数
/// 
/// # 参数
/// * `seconds` - 等待的秒数（支持小数）
/// 
/// 使用 from_secs_f32 支持浮点数秒数，提供更精确的时间控制
async fn timeout(seconds: f32) {
    tokio::time::sleep(Duration::from_secs_f32(seconds)).await;
}

/// 程序入口点
/// #[tokio::main] 宏将 async main 函数转换为同步的 main 函数
/// 并自动创建 Tokio 运行时来执行异步代码
/// 文档: https://docs.rs/tokio/latest/tokio/attr.main.html
#[tokio::main]
async fn main() {
    // 第一个 select! 演示：竞争执行两个异步任务
    // do_work() 需要 2 秒，timeout(1.0) 需要 1 秒
    // 因此 timeout 会先完成，演示了 select! 的"竞争"特性
    println!("=== 演示 1: 竞争执行 ===");
    select! {
        _ = do_work() => println!("do work finished first"),
        _ = timeout(1.0) => println!("timeout finished first"),
    }

    println!("\n=== 演示 2: 多通道消息处理 ===");
    
    // 创建 MPSC（多生产者单消费者）通道
    // 缓冲区大小为 1，意味着最多可以缓存 1 条未被接收的消息
    // 文档: https://docs.rs/tokio/latest/tokio/sync/mpsc/fn.channel.html
    let (tx, rx) = mpsc::channel::<u32>(1);

    // 创建广播通道
    // 缓冲区大小为 1，所有接收者都会收到相同的消息
    // 文档: https://docs.rs/tokio/latest/tokio/sync/broadcast/fn.channel.html
    let (broadcast_tx, broadcast_rx) = broadcast::channel::<u32>(1);

    // 在新的异步任务中启动接收器
    // tokio::spawn 创建一个新的异步任务，与当前任务并发执行
    // 文档: https://docs.rs/tokio/latest/tokio/fn.spawn.html
    tokio::spawn(receiver(rx, broadcast_rx));

    // 循环发送消息到两种不同的通道
    // 偶数发送到 MPSC 通道，奇数发送到广播通道
    // 这样可以观察 select! 如何处理来自不同通道的消息
    for count in 0..10 {
        if count % 2 == 0 {
            // 发送到 MPSC 通道（异步操作，可能会阻塞直到有接收者）
            tx.send(count).await.unwrap();
            println!("Sent {} to MPSC channel", count);
        } else {
            // 发送到广播通道（同步操作，立即返回）
            broadcast_tx.send(count).unwrap();
            println!("Sent {} to broadcast channel", count);
        }
        // 每次发送后等待 1 秒，让接收器有时间处理消息
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    
    // 等待一段时间确保所有消息都被处理
    println!("\nWaiting for remaining messages to be processed...");
    tokio::time::sleep(Duration::from_secs(2)).await;
}
