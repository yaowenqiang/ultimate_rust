// DashMap: 并发安全的哈希表，支持多线程无锁访问
// 官方文档: https://docs.rs/dashmap/latest/dashmap/
use dashmap::DashMap;

// once_cell: 提供线程安全的延迟初始化功能
// 官方文档: https://docs.rs/once_cell/latest/once_cell/
use once_cell::sync::Lazy;

// 创建全局共享的并发安全哈希表
// Lazy 确保 DashMap 只在首次访问时初始化，且线程安全
// DashMap<u32, u32> 表示键和值都是 32 位无符号整数
static SHARED: Lazy<DashMap<u32, u32>> = Lazy::new(|| DashMap::new());

fn main() {
    // 创建 100 个并发线程，每个线程处理一个唯一的数字 n
    for n in 1..=100 {
        // spawn 创建新线程，move 将 n 的所有权转移到闭包中
        // 线程文档: https://doc.rust-lang.org/std/thread/fn.spawn.html
        std::thread::spawn(move || {
            // 无限循环，持续操作共享哈希表
            loop {
                // 尝试获取键 n 对应的可变引用
                // get_mut 返回 Option<RefMut<V>>，如果键存在则返回 Some
                if let Some(mut v) = SHARED.get_mut(&n) {
                    // 如果键存在，将其值递增 1
                    *v += 1;
                } else {
                    // 如果键不存在，插入新的键值对 (n, n)
                    // insert 方法是原子操作，线程安全
                    SHARED.insert(n, n);
                }
            }
        });
    }

    // 主线程休眠 5 秒，让工作线程有时间执行操作
    // Duration 文档: https://doc.rust-lang.org/std/time/struct.Duration.html
    std::thread::sleep(std::time::Duration::from_secs(5));

    // 打印最终的哈希表内容
    // {:#?} 是美化打印格式，显示结构化的调试信息
    println!("{SHARED:#?}");
}
