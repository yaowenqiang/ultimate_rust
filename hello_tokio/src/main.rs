//! hello_tokio 示例
//! 
//! 本示例展示了如何：
//! - 使用 `#[tokio::main]` 宏启动 Tokio 运行时（单线程 current_thread 风格）
//! - 通过 `tokio::spawn` 在运行时中并发地运行多个异步任务
//! - 使用 `tokio::join!` 等待多个任务完成
//! - 在协作式调度下使用 `tokio::task::yield_now()` 主动让出执行权
//!
//! 相关文档：
//! - Tokio 入门: https://tokio.rs/tokio/tutorial
//! - `#[tokio::main]` 宏: https://docs.rs/tokio/latest/tokio/attr.main.html
//! - 运行时（Runtime）: https://docs.rs/tokio/latest/tokio/runtime/index.html
//! - `tokio::spawn`：https://docs.rs/tokio/latest/tokio/fn.spawn.html
//! - `tokio::join!`：https://docs.rs/tokio/latest/tokio/macro.join.html
//! - `yield_now`（协作式让出）：https://docs.rs/tokio/latest/tokio/task/fn.yield_now.html
//! - 协作式调度（Cooperative Scheduling）说明：
//!   https://docs.rs/tokio/latest/tokio/task/index.html#cooperative-scheduling

/// 一个简单的异步函数，打印信息并返回一个数值。
///
/// 注意：`async fn` 返回一个实现 `Future` 的匿名类型，只有在 `.await`
/// 或被运行时 `block_on`（如手工构建的 runtime）驱动时才会被执行。
async fn hello() -> u32 {
    println!("hello tokio");
    3
}

/// 另一个异步函数，与 [`hello`] 类似，用于演示 `tokio::join!`/`tokio::spawn` 的并发性。
async fn hello_tokio() -> u32 {
    println!("hello tokio2");
    4
}

/// 一个“滴答器”任务，用来演示协作式调度。
///
/// 在 Tokio 中，异步任务运行在执行器上。为了确保长时间运行（或计算密集）的任务
/// 不会独占线程导致其他任务饥饿，Tokio 使用协作式调度。调用
/// [`tokio::task::yield_now()`] 会主动让出当前任务的执行权，使运行时有机会调度
/// 其他就绪的任务。
///
/// 参考：`yield_now` 文档
/// https://docs.rs/tokio/latest/tokio/task/fn.yield_now.html
async fn ticker() {
    for i in 0..10 {
        println!("tick {i}");
        // 主动让出执行权，帮助其他任务获得调度机会
        tokio::task::yield_now().await;
    }
}

/// Tokio 的入口点。
///
/// 这里我们使用 `#[tokio::main(flavor = "current_thread")]` 来构建一个“当前线程”
/// 风格的运行时：
/// - current_thread：所有异步任务都在主线程上轮转执行，便于教学与可预测的日志顺序；
/// - multi_thread：多工作线程的线程池，适合 I/O 密集或大量并发场景。
///
/// 如果需要切换为多线程运行时，可以将下方被注释的 `#[tokio::main()]`
/// 或 `runtime::Builder::new_multi_thread()` 的示例解除注释并使用。
#[tokio::main(flavor = "current_thread")]
// #[tokio::main()] // 默认即 multi_thread 版本（启用多线程调度器）
async fn main() {
    // ——— 手动构建运行时的两种方式（演示用，通常直接用 #[tokio::main] 更简洁）———
    // use tokio::runtime;
    // let rt = runtime::Builder::new_current_thread()
    //     .enable_all() // 启用时间、I/O 等所有 Tokio 子系统
    //     .build()
    //     .unwrap();
    // // 手动驱动一个 future：
    // rt.block_on(hello());

    // let rt = runtime::Builder::new_multi_thread()
    //     .enable_all()
    //     .worker_threads(4)
    //     .build()
    //     .unwrap();
    // rt.block_on(hello());

    // ——— 基础 await 与并发等待示例 ———
    // 1) 顺序执行：
    // let v1 = hello().await;
    // let v2 = hello_tokio().await;
    // println!("sequential: {v1}, {v2}");

    // 2) 并发等待（不生成新任务，单线程/多线程下均并发推进）：
    // let results = tokio::join!(hello_tokio(), hello());
    // println!("join!: {results:?}");
    // let (one, two) = results;
    // println!("{one:?} {two:?}");

    // 3) 通过 tokio::spawn 将 Future 移交给运行时调度（生成独立任务）：
    //    注意：被 `spawn` 的任务要求 `Send + 'static`（current_thread 运行时在部分场景
    //    下允许非 Send，但建议保证 Send 以便将来迁移到 multi_thread 更容易）。
    //    文档：https://docs.rs/tokio/latest/tokio/fn.spawn.html

    let _ = tokio::join!(
        tokio::spawn(hello()),   // 返回 JoinHandle<u32>
        tokio::spawn(ticker()),  // 返回 JoinHandle<()>
        tokio::spawn(ticker()),  // 再起一个滴答器任务
    );

    // 提示：如果你需要逐个收集 JoinHandle 的结果，也可以使用 JoinSet：
    // https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html

    println!("finished main");
}
