# Hello Tokio - Tokio 异步编程学习示例

本项目包含两个版本的 Tokio 示例，旨在帮助学习者理解异步编程的基础概念和性能优势。

## 📁 文件结构

```
hello_tokio/
├── src/
│   ├── main.rs              # 基础版本：展示 Tokio 基本概念
│   └── main_performance.rs  # 性能对比版本：展示异步编程的性能优势
├── Cargo.toml
└── README.md                # 本文件
```

## 🎯 两个版本的区别

### 🔰 基础版本 (`main.rs`)

**学习目标：** 理解 Tokio 异步编程的基本概念

**特点：**
- 使用 `#[tokio::main]` 宏启动运行时
- 演示异步函数的基本用法
- 展示 `tokio::spawn` 和 `tokio::join!` 的使用
- 通过 `yield_now()` 理解协作式调度
- 详细的中文注释和文档链接

**运行方式：**
```bash
cargo run
```

**输出示例：**
```
hello tokio
tick 0
tick 0
tick 1
tick 1
...
finished main
```

### 🚀 性能对比版本 (`main_performance.rs`)

**学习目标：** 量化理解异步编程的性能优势

**特点：**
- 通过模拟 I/O 任务展示同步 vs 异步的性能差异
- 对比串行执行 vs 并发执行的耗时
- 展示混合负载（I/O + CPU）下的协作式调度
- 提供详细的性能分析和测试结果

**运行方式：**
```bash
# 方法 1: 临时替换主文件（推荐用于性能测试）
cp src/main_performance.rs src/main.rs
cargo run --release
cp src/main.rs src/main_performance.rs  # 恢复备份

# 方法 2: 直接编译运行（需要手动处理依赖）
rustc src/main_performance.rs --edition 2024 [依赖参数...]
```

**性能测试结果示例：**
```
🎯 === Tokio 异步性能对比测试 ===

🔄 执行同步版本 (8 个任务，各耗时 100ms)
📊 同步版本总耗时: 805.9365ms

🚀 执行异步版本 - spawn 并发 (8 个任务，各耗时 100ms) 
📊 异步 spawn 版本总耗时: 111.5101ms

🚀 === 性能提升分析 ===
Spawn 方式性能提升: 7.2x
Join_all 方式性能提升: 7.1x
```

## 🎓 学习路径建议

### 第一步：理解基础概念
1. 运行基础版本 (`main.rs`)
2. 阅读代码中的详细注释
3. 理解以下核心概念：
   - `async fn` 和 `Future`
   - `await` 关键字
   - `tokio::spawn` vs `tokio::join!`
   - 协作式调度

### 第二步：体验性能优势
1. 运行性能对比版本 (`main_performance.rs`)
2. 观察同步 vs 异步的执行时间差异
3. 理解为什么异步编程在 I/O 密集型场景下有巨大优势

### 第三步：深入理解
1. 修改性能测试参数（任务数量、延迟时间）
2. 尝试不同的并发模式
3. 阅读相关文档和最佳实践

## 📚 相关文档

### 官方文档
- [Tokio 官方教程](https://tokio.rs/tokio/tutorial)
- [异步编程概念](https://rust-lang.github.io/async-book/)
- [Tokio 性能指南](https://tokio.rs/tokio/topics/performance)

### API 文档
- [`#[tokio::main]` 宏](https://docs.rs/tokio/latest/tokio/attr.main.html)
- [`tokio::spawn`](https://docs.rs/tokio/latest/tokio/fn.spawn.html)
- [`tokio::join!`](https://docs.rs/tokio/latest/tokio/macro.join.html)
- [`yield_now`](https://docs.rs/tokio/latest/tokio/task/fn.yield_now.html)

## 🔧 开发命令

```bash
# 基本运行
cargo run

# 性能测试（发布模式）
cargo run --release

# 代码检查
cargo check
cargo clippy

# 运行测试
cargo test

# 从父目录运行
cargo run -p hello_tokio
```

## 💡 关键洞察

1. **异步 ≠ 并行**：异步是关于任务的交错执行，并行是关于同时执行
2. **I/O 优势**：异步编程在 I/O 密集型任务中表现出色
3. **协作式调度**：需要主动让出执行权（`yield_now()`）
4. **运行时选择**：`current_thread` vs `multi_thread` 适用不同场景
5. **性能测量**：实际性能提升取决于工作负载特性

## 🚀 进阶学习

完成本项目后，建议继续学习：

1. **真实 I/O**：HTTP 客户端、数据库操作
2. **流式处理**：`tokio::stream` 和背压控制
3. **并发模式**：channels、select、timeout
4. **错误处理**：异步上下文中的错误传播
5. **监控调试**：tracing、async-profiler 等工具

祝学习愉快！🎉