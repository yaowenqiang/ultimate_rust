# Rust Unsafe 代码演示项目

这是一个演示 Rust 中 `unsafe` 代码使用的教育项目。通过实际的代码示例，展示了 Rust 安全编程与 unsafe 代码之间的区别和注意事项。

## 🎯 学习目标

- 理解 Rust 的内存安全保证
- 学习何时以及如何使用 unsafe 代码
- 掌握 unsafe 代码的风险和最佳实践
- 了解 Rust 的边界检查机制

## 📁 项目结构

```
unsafe_demo/
├── Cargo.toml          # 项目配置文件
├── README.md           # 项目说明文档
└── src/
    └── main.rs         # 主要演示代码
```

## 🚀 运行项目

```bash
cargo run
```

**⚠️ 警告**：这个项目包含故意的内存安全违规代码，仅用于教育目的。在生产环境中切勿模仿这种写法。

## 📚 代码说明

### 主要演示内容

1. **安全的向量访问**
   - 使用索引操作符 `[]`：会进行边界检查，越界时 panic
   - 使用 `get()` 方法：返回 `Option<T>`，安全处理越界情况

2. **unsafe 访问方式**
   - 使用 `get_unchecked()` 方法：不进行边界检查，可能导致未定义行为
   - 演示 unsafe 函数的定义和调用

### 关键概念

- **边界检查 (Bounds Checking)**：Rust 在运行时检查数组/向量访问是否在有效范围内
- **未定义行为 (Undefined Behavior)**：程序行为无法预测，可能导致崩溃或安全漏洞
- **内存安全**：Rust 的核心特性，防止常见的内存错误

## 🔗 相关文档

### 官方文档
- [Rust Book - Unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)
- [Rustonomicon - The Rust Book of Unsafety](https://doc.rust-lang.org/nomicon/)
- [Rust Reference - Unsafe Blocks](https://doc.rust-lang.org/reference/unsafe-blocks.html)

### 中文资源
- [Rust语言圣经(Rust Course) - Unsafe Rust](https://course.rs/advance/unsafe.html)
- [Rust By Example - Unsafe](https://rustwiki.org/zh-CN/rust-by-example/unsafe.html)
- [Rust 中文文档 - Unsafe 关键字](https://www.rustwiki.org.cn/keyword/unsafe.html)

### 最佳实践指南
- [Rust Unsafe Guidelines](https://rust-lang.github.io/unsafe-code-guidelines/)
- [Writing Unsafe Rust](https://doc.rust-lang.org/nightly/nomicon/writing-unsafe-rust.html)

## ⚠️ 重要提醒

- `unsafe` 不等于不安全，而是告诉编译器"我知道我在做什么"
- 使用 unsafe 时必须确保手动维护 Rust 通常提供的安全保证
- 仅在绝对必要且了解所有风险时才使用 unsafe 代码
- 优先寻找安全的替代方案

## 🛠️ 扩展练习

1. 尝试修复代码中的安全问题
2. 实现一个安全的包装器函数
3. 探索其他 unsafe 使用场景（如裸指针、FFI 等）

## 📄 许可证

本项目仅用于教育目的，请遵循相关的开源许可证。