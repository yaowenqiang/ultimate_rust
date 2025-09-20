//! # Mutex Demo - 互斥锁基础演示
//! 
//! 这个项目演示了如何使用 `std::sync::Mutex` 来保护共享数据，
//! 确保多个线程安全地访问和修改同一个数据结构。
//! 
//! ## 核心概念
//! - Mutex (互斥锁): 确保同一时间只有一个线程能访问被保护的数据
//! - 静态共享状态: 使用 `static` 关键字创建全局共享的数据
//! - 线程安全: 通过锁机制避免数据竞争
//! 
//! ## 相关文档
//! - [Rust Book - 共享状态并发](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
//! - [std::sync::Mutex 文档](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
//! - [std::thread 模块文档](https://doc.rust-lang.org/std/thread/index.html)
//! - [Rust 并发编程指南](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

use std::sync::Mutex;

/// 全局共享的线程安全数据结构
/// 
/// 使用 Mutex 包装 Vec 来确保多线程安全访问。
/// `static` 关键字使这个变量在整个程序生命周期内都存在，
/// 所有线程都可以访问同一个实例。
/// 
/// 文档: https://doc.rust-lang.org/std/sync/struct.Mutex.html
static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

/// 演示基本的 Mutex 使用模式
/// 
/// 创建多个线程同时向共享的 Vec 中添加数据，
/// 通过 Mutex 确保操作的线程安全性。
fn main() {
    println!("🔐 Mutex Demo - 互斥锁基础演示");
    println!("创建 10 个线程，每个线程向共享 Vec 添加一个元素\n");
    
    // 存储所有线程句柄的容器
    let mut handles = Vec::new();
    
    // 创建 10 个并发线程
    for thread_id in 0..10 {
        // std::thread::spawn 创建新的操作系统线程
        // move 闭包捕获 thread_id 的所有权
        // 文档: https://doc.rust-lang.org/std/thread/fn.spawn.html
        let handle = std::thread::spawn(move || {
            // 获取互斥锁
            // lock() 方法会阻塞当前线程直到获得锁的独占访问权
            // unwrap() 处理可能的 PoisonError (在这个简单示例中是安全的)
            // 文档: https://doc.rust-lang.org/std/sync/struct.Mutex.html#method.lock
            let mut lock = NUMBERS.lock().unwrap();
            
            // 在锁的保护下安全地修改共享数据
            // 此时只有当前线程能访问 Vec，其他线程必须等待
            lock.push(thread_id + 1); // 添加线程ID+1以便区分
            
            println!("线程 {} 成功添加数据", thread_id + 1);
            
            // 当 lock 变量超出作用域时，互斥锁自动释放
            // 其他等待的线程可以获取锁并继续执行
        });
        
        handles.push(handle);
    }
    
    println!("\n等待所有线程完成...");
    
    // 等待所有线程完成执行
    // join() 确保主线程等待所有子线程完成
    // into_iter() 获取 handles 的所有权，避免借用检查问题
    // 文档: https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#method.join
    handles.into_iter().for_each(|handle| {
        handle.join().expect("线程执行失败");
    });
    
    println!("\n📊 最终结果:");
    
    // 获取最终的数据进行显示
    // 再次获取锁来安全地读取共享数据
    let lock = NUMBERS.lock().unwrap();
    
    // {:#?} 是 pretty-print 格式化，显示更易读的输出
    println!("共享 Vec 内容: {:#?}", *lock);
    println!("总元素数量: {}", lock.len());
    
    // 验证所有元素都被正确添加
    let expected_sum: u32 = (1..=10).sum();
    let actual_sum: u32 = lock.iter().sum();
    
    if actual_sum == expected_sum {
        println!("✅ 验证通过: 所有数据都被正确添加 (和为 {})", actual_sum);
    } else {
        println!("❌ 数据异常: 预期和为 {}，实际和为 {}", expected_sum, actual_sum);
    }
    
    // 当 lock 超出作用域时，最后一次自动释放锁
    
    println!("\n💡 技术要点:");
    println!("   🔹 Mutex: 确保同一时间只有一个线程能访问数据");
    println!("   🔹 lock(): 获取互斥锁，可能阻塞线程");
    println!("   🔹 自动释放: 锁变量超出作用域时自动释放");
    println!("   🔹 线程安全: 避免数据竞争，确保结果可预测");
    
    println!("\n🔗 相关文档:");
    println!("   📖 https://doc.rust-lang.org/std/sync/struct.Mutex.html");
    println!("   📖 https://doc.rust-lang.org/book/ch16-03-shared-state.html");
}
