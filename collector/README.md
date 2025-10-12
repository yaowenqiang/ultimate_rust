# 数据收集器 (Collector) - 分布式系统监控组件

## 项目概述

这是一个用 Rust 实现的系统性能数据收集器，作为分布式监控系统的客户端组件。该项目展示了 Rust 在系统编程、网络通信和并发处理方面的强大能力。

## 核心功能

- 🖥️ **系统监控**: 实时收集 CPU 使用率和内存使用情况
- 🌐 **网络通信**: 通过 TCP 连接将数据发送到中央服务器
- 🆔 **身份管理**: 使用持久化 UUID 唯一标识每个收集器实例
- 🔄 **并发架构**: 采用生产者-消费者模式的多线程设计

## 技术架构

### 组件设计

```
┌─────────────────┐    通道 (MPSC)    ┌─────────────────┐
│   数据收集线程   │ ───────────────► │    主线程       │
│                │                  │                │
│ • 系统性能监控   │                  │ • 网络通信      │
│ • 定时采样      │                  │ • 数据发送      │
│ • 数据计算      │                  │ • 错误处理      │
└─────────────────┘                  └─────────────────┘
         │                                     │
         ▼                                     ▼
   sysinfo 库                          TCP 网络连接
         │                                     │
         └─────────────── 中央服务器 ◄──────────┘
```

### 数据流程

1. **初始化**: 获取持久化 UUID，创建通信通道
2. **数据收集**: 后台线程每秒收集系统性能数据
3. **数据传输**: 主线程通过 TCP 连接发送数据到服务器
4. **持续运行**: 程序持续运行直到外部终止

## 学习目标

通过这个项目，你将学会：

### Rust 核心概念
- **所有权系统**: `move` 关键字在闭包中的使用
- **生命周期**: 多线程间的数据共享
- **错误处理**: `Result` 类型和 `unwrap()` 的使用场景
- **模式匹配**: `if let` 和 `while let` 的应用

### 并发编程
- **多线程**: `std::thread::spawn()` 的使用
- **线程间通信**: MPSC (多生产者单消费者) 通道
- **同步原语**: 通道的安全数据传递机制
- **生产者-消费者模式**: 经典的并发设计模式

### 系统编程
- **系统信息获取**: 使用 `sysinfo` 库监控系统资源
- **网络编程**: TCP 客户端的实现
- **文件 I/O**: UUID 的持久化存储
- **时间处理**: 精确的采样间隔控制

### 实用技能
- **依赖管理**: Cargo.toml 的配置
- **序列化**: 数据的编码和传输
- **协议设计**: 自定义二进制协议
- **性能优化**: Release 模式的重要性

## 依赖库详解

### sysinfo
- **用途**: 跨平台系统信息获取
- **功能**: CPU、内存、进程、磁盘等系统资源监控
- **文档**: https://docs.rs/sysinfo/latest/sysinfo/
- **替代方案**: `heim`, `psutil` (Python 移植)

### uuid
- **用途**: 生成和管理唯一标识符
- **版本**: UUID v4 (随机生成)
- **特性**: `fast-rng` 快速随机数生成
- **文档**: https://docs.rs/uuid/latest/uuid/
- **规范**: RFC 4122

### shared_data (本地库)
- **用途**: 共享的数据结构和网络协议
- **内容**: 命令枚举、编码/解码函数
- **序列化**: bincode 二进制序列化
- **协议**: 自定义 TCP 协议格式

## 运行指南

### 基本运行
```bash
# 在 collector 目录下
cargo run

# 推荐使用 release 模式以获得更好性能
cargo run --release
```

### 开发调试
```bash
# 检查代码
cargo check

# 格式化代码
cargo fmt

# 静态分析
cargo clippy

# 运行测试
cargo test
```

### 性能测试
```bash
# 多次运行测试一致性
for i in {1..5}; do
    echo "运行 $i:"
    time cargo run --release
done
```

## 文件结构

```
collector/
├── src/
│   └── main.rs          # 主程序文件
├── Cargo.toml           # 项目配置和依赖
├── README.md           # 项目文档
└── uuid                # 持久化 UUID 文件 (运行时生成)
```

## 网络协议

### 数据包格式
```
┌─────────────┬─────────────┬─────────────┬─────────────┬─────────────┬─────────────┐
│ Magic Number│ Version     │ Timestamp   │ Payload Size│ Payload     │ CRC32       │
│   (2 bytes) │   (2 bytes) │   (4 bytes) │   (4 bytes) │  (variable) │   (4 bytes) │
└─────────────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┘
```

### 字段说明
- **Magic Number**: `1234` (协议标识符)
- **Version**: `1` (协议版本)
- **Timestamp**: Unix 时间戳 (秒)
- **Payload Size**: 数据载荷大小
- **Payload**: bincode 序列化的 CollectorCommandV1
- **CRC32**: 数据完整性校验

## 扩展练习

### 初级练习
1. **添加更多监控指标**: 磁盘使用率、网络流量、进程数量
2. **改进错误处理**: 使用 `?` 操作符替代 `unwrap()`
3. **日志系统**: 集成 `log` 和 `env_logger` 库
4. **配置文件**: 支持 TOML/YAML 配置文件

### 中级练习
1. **重试机制**: 网络连接失败时的自动重试
2. **数据压缩**: 在传输前压缩数据以节省带宽
3. **心跳机制**: 定期发送心跳包保持连接
4. **优雅关闭**: 处理 SIGINT/SIGTERM 信号

### 高级练习
1. **TLS 加密**: 使用 rustls 实现 TLS 连接
2. **批量发送**: 累积多个数据点后批量发送
3. **多协议支持**: 支持 HTTP/gRPC 等多种传输协议
4. **插件系统**: 支持动态加载的监控插件

## 相关资源

### Rust 官方文档
- [Rust 程序设计语言](https://doc.rust-lang.org/book/)
- [Rust 标准库](https://doc.rust-lang.org/std/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)

### 并发编程资源
- [Rust 并发编程](https://doc.rust-lang.org/book/ch16-00-fearless-concurrency.html)
- [ tokio 教程](https://tokio.rs/tokio/tutorial)
- [Crossbeam](https://docs.rs/crossbeam/) - 高级并发工具

### 网络编程资源
- [Rust 网络编程](https://doc.rust-lang.org/std/net/)
- [TCP/IP 协议详解](https://tools.ietf.org/html/rfc793)
- [REST API 设计](https://restfulapi.net/)

### 系统监控资源
- [系统监控概念](https://en.wikipedia.org/wiki/System_monitor)
- [性能分析工具](https://github.com/giampaolo/psutil)
- [时间序列数据库](https://prometheus.io/)

### 设计模式资源
- [生产者-消费者模式](https://en.wikipedia.org/wiki/Producer%E2%80%93consumer_problem)
- [并发设计模式](https://en.wikipedia.org/wiki/Concurrency_pattern)
- [网络编程模式](https://en.wikipedia.org/wiki/Network_programming)

## 故障排除

### 常见问题

**Q: 程序无法连接到服务器**
A: 检查服务器是否在 `127.0.0.1:9004` 运行，确认防火墙设置

**Q: CPU 使用率始终为 0**
A: 需要等待至少 1 秒让 sysinfo 库计算 CPU 使用率

**Q: 编译失败提示找不到 shared_data**
A: 确保在 ultimate_rust 工作空间根目录运行 `cargo build`

**Q: 程序运行缓慢**
A: 使用 `cargo run --release` 编译优化版本

### 调试技巧

1. **添加调试输出**:
   ```rust
   println!("Debug: {:?}", data);
   ```

2. **使用调试器**:
   ```bash
   rust-gdb target/debug/collector
   ```

3. **性能分析**:
   ```bash
   cargo build --release
   perf record ./target/release/collector
   perf report
   ```

## 贡献指南

欢迎提交 Issue 和 Pull Request！

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启 Pull Request

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 更新日志

### v0.1.0 (当前版本)
- ✨ 基本的系统性能数据收集
- 🌐 TCP 网络数据传输
- 🆔 UUID 持久化身份管理
- 🔄 多线程并发架构