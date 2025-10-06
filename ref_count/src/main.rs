// Rust 智能指针与多线程共享数据示例
//
// 本程序演示了 Rust 中不同的智能指针类型和它们在多线程环境中的使用：
// - Rc<T>: 引用计数指针，用于单线程环境
// - Arc<T>: 原子引用计数指针，用于多线程环境
// - RefCell<T>: 内部可变性，用于单线程环境
// - Mutex<T>: 互斥锁，用于多线程环境
//
// 相关文档链接：
// - Rc: https://doc.rust-lang.org/std/rc/struct.Rc.html
// - Arc: https://doc.rust-lang.org/std/sync/struct.Arc.html
// - RefCell: https://doc.rust-lang.org/std/cell/struct.RefCell.html
// - Mutex: https://doc.rust-lang.org/std/sync/struct.Mutex.html
// - 智能指针指南: https://doc.rust-lang.org/book/ch15-00-smart-pointers.html
// - 并发编程指南: https://doc.rust-lang.org/book/ch16-00-fearless-concurrency.html
// - 所有权系统: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
// - 多线程间共享状态: https://doc.rust-lang.org/rust-by-example/std_misc/threads.html

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

/// 一个演示资源释放的结构体
///
/// 当结构体被销毁时，会自动调用 Drop trait 的 drop 方法
/// 这对于观察智能指针何时释放资源很有用
#[derive(Debug)]
struct Droppable(i32);

impl Droppable {
    /// 创建一个新的 Droppable 实例
    ///
    /// # 参数
    /// * `n` - 要存储的整数值
    fn new(n: i32) -> Self {
        println!("正在构建 Droppable 实例: {}", n);
        Self(n)
    }
}

impl Drop for Droppable {
    /// 当实例被销毁时调用
    ///
    /// 这个方法会在智能指针的引用计数降到 0 时自动调用
    fn drop(&mut self) {
        println!("正在销毁 Droppable 实例: {}", self.0);
    }
}

/// 演示 Rc 智能指针的所有权转移
///
/// Rc 允许多个所有者共享同一个数据，但在单线程环境中使用
/// 当所有 Rc 引用都被销毁时，底层数据才会被释放
fn move_me(x: Rc<Droppable>) {
    println!("Rc 指针被移动: {}", x.0);
}

/// 用于 Arc + Mutex 组合的共享数据结构
///
/// 这种组合允许在多线程环境中安全地共享和修改数据
struct SharedData(String);

/// 使用 Mutex 包装的共享数据结构
///
/// Mutex 提供了运行时的互斥访问，确保同一时间只有一个线程可以访问数据
struct SharedData2 {
    data: Mutex<String>,
}

/// 使用 RefCell 提供内部可变性的数据结构
///
/// RefCell 允许在编译时不可变的情况下，在运行时进行可变借用
/// 注意：RefCell 只能在单线程环境中使用
struct MyData {
    data: RefCell<String>,
}

impl MyData {
    /// 创建一个新的 MyData 实例
    ///
    /// 内部使用 RefCell 包装字符串，提供内部可变性
    fn new() -> Self {
        Self {
            data: RefCell::new("Hello".to_string()),
        }
    }
}

/// 演示 RefCell 的使用
///
/// 通过 borrow_mut() 获取可变引用，修改内部数据
/// RefCell 在运行时检查借用规则，违反规则会 panic
fn move_data(arc_data: Arc<MyData>) {
    let mut refcell_data = arc_data.data.borrow_mut(); // 获取可变借用
    refcell_data.push_str("World"); // 修改数据
}

impl SharedData2 {
    /// 创建一个新的 SharedData2 实例
    ///
    /// 使用 Mutex 包装字符串，支持多线程访问
    fn new(s: &str) -> Self {
        Self {
            data: Mutex::new(s.to_string()),
        }
    }
}
fn main() {
    println!("=== Rc 智能指针演示 ===");
    // Rc: 引用计数智能指针，用于单线程环境
    // 当最后一个 Rc 被销毁时，底层数据会被自动清理
    let rc_shared_data = Rc::new(Droppable::new(1));

    // 创建多个 Rc 引用，引用计数会增加
    {
        let _rc_clone1 = rc_shared_data.clone(); // 引用计数: 2
        let _rc_clone2 = rc_shared_data.clone(); // 引用计数: 3
        let _rc_clone3 = rc_shared_data.clone(); // 引用计数: 4

        println!("在作用域内创建了3个额外的Rc引用");
        // 当离开作用域时，_rc_clone1, _rc_clone2, _rc_clone3 被销毁，引用计数会减少
    }
    // 此时引用计数应该回到 1（只有 rc_shared_data 存在）

    println!("\n=== Arc 智能指针多线程演示 ===");
    // Arc: 原子引用计数智能指针，用于多线程环境
    // 提供线程安全的引用计数操作
    let arc_shared_data = Arc::new(Droppable::new(1));
    move_me(rc_shared_data.clone()); // 演示 Rc 的所有权转移

    // 创建多个线程，每个线程都持有 Arc 的克隆
    let mut arc_threads: Vec<thread::JoinHandle<()>> = Vec::new();
    for thread_id in 0..10 {
        let arc_data_clone = arc_shared_data.clone(); // 每次克隆都会增加引用计数
        arc_threads.push(thread::spawn(move || {
            println!("线程 {} 访问共享数据: {:?}", thread_id, arc_data_clone);
        }));
    }

    // 等待所有线程完成
    for thread_handle in arc_threads {
        thread_handle.join().unwrap();
    }

    println!("\n当前 Rc 状态: {:?}", rc_shared_data);
    println!("程序即将退出，观察资源清理顺序");

    println!("\n=== Arc + Mutex 可变共享数据演示 ===");
    // Arc + Mutex 组合：多线程安全的可变共享数据
    // Arc 提供线程安全的引用计数，Mutex 提供线程安全的可变访问
    let arc_mutex_data = Arc::new(Mutex::new(SharedData("Hello".to_string())));

    let mut mutex_threads = Vec::new();

    // 创建多个线程，每个线程都会修改共享数据
    for worker_id in 0..10 {
        let arc_mutex_clone = arc_mutex_data.clone();
        mutex_threads.push(thread::spawn(move || {
            // lock() 获取互斥锁，确保同时只有一个线程可以访问数据
            let mut shared_string = arc_mutex_clone.lock().unwrap();
            shared_string.0.push_str(&format!(" {}", worker_id));
            // 锁在离开作用域时自动释放
        }));
    }

    // 等待所有线程完成修改
    for thread_handle in mutex_threads {
        thread_handle.join().unwrap();
    }

    // 读取最终结果
    let final_data = arc_mutex_data.lock().unwrap();
    println!("最终的共享数据: {}", final_data.0);

    println!("\n=== Arc + 结构体内置 Mutex 演示 ===");
    // 直接在结构体中使用 Mutex，提供更好的封装性
    let struct_mutex_data = Arc::new(SharedData2::new("hello"));
    let mut struct_mutex_threads = Vec::new();

    // 多个线程同时修改共享数据
    for task_id in 0..10 {
        let struct_mutex_clone = struct_mutex_data.clone();
        struct_mutex_threads.push(thread::spawn(move || {
            // 直接访问结构体内部的 Mutex
            let mut struct_data = struct_mutex_clone.data.lock().unwrap();
            struct_data.push_str(&format!(" {}", task_id));
        }));
    }

    // 等待所有线程完成
    for thread_handle in struct_mutex_threads {
        thread_handle.join().unwrap();
    }

    // 读取最终结果
    let struct_final_data = struct_mutex_data.data.lock().unwrap();
    println!("SharedData2 最终内容: {}", struct_final_data);

    println!("\n=== Arc + RefCell 演示（单线程） ===");
    // 注意：RefCell + Arc 虽然可以编译，但要小心使用
    // RefCell 不是线程安全的，通常应该避免在多线程中使用
    let arc_refcell_data = Arc::new(MyData::new());

    // 在主线程中调用 move_data 函数
    move_data(arc_refcell_data.clone());

    // 使用 borrow() 获取不可变引用来读取数据
    let refcell_result = arc_refcell_data.data.borrow();
    println!("MyData 最终内容: {}", refcell_result);

    println!("\n=== 程序结束 ===");
    println!("观察各个智能指针的资源清理顺序");
}
