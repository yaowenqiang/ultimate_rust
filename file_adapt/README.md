# file_adapt - Rust 异步流式文件服务器

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tokio](https://img.shields.io/badge/tokio-1.48+-blue.svg)](https://tokio.rs)
[![Axum](https://img.shields.io/badge/axum-0.8+-red.svg)](https://github.com/tokio-rs/axum)

这是一个使用 Rust 和 axum 框架构建的简单但功能完整的异步流式文件服务器。该项目演示了现代 Rust 异步编程的核心概念，包括异步文件 I/O、流式响应处理和 Web 服务构建。

## 🚀 功能特性

- **异步文件读取** - 使用 tokio 非阻塞读取文件，不阻塞线程
- **流式响应** - 逐行处理文件内容，适合大文件传输
- **实时内容转换** - 将文件内容实时转换为大写字母
- **现代 Rust 实践** - 使用 2024 版本 Rust 和最佳实践
- **完整的错误处理** - 优雅处理文件不存在等错误情况
- **标准 HTTP 响应** - 设置正确的响应头和状态码

## 🛠 技术栈

| 组件 | 版本 | 用途 |
|------|------|------|
| **axum** | 0.8.6 | 现代化 Web 框架 |
| **tokio** | 1.48.0 | 异步运行时 |
| **tokio-stream** | 0.1.17 | 异步流处理 |
| **tokio-util** | 0.7.16 | 异步工具库 |
| **pin-project-lite** | 0.2.16 | 轻量级 pin 项目 |
| **futures** | 0.3.31 | 异步未来处理 |

## 📦 安装与运行

### 前置要求

- Rust 1.70 或更高版本（推荐使用最新稳定版）
- Cargo 包管理器

### 编译运行

```bash
# 克隆项目或进入项目目录
cd file_adapt

# 编译项目
cargo build --release

# 运行服务器
cargo run

# 或者使用 release 模式运行（推荐用于生产环境）
cargo run --release
```

服务器启动后，将在 `http://localhost:3000` 监听请求。

## 🔍 使用示例

### 基本请求

```bash
# 使用 curl 访问服务
curl http://localhost:3000

# 查看响应头
curl -i http://localhost:3000

# 保存到文件
curl -o output.txt http://localhost:3000
```

### 浏览器访问

直接在浏览器中打开 `http://localhost:3000`，浏览器会提示下载文件。

### 响应内容

服务器会读取项目根目录下的 `Cargo.toml` 文件，将其内容转换为大写后返回。示例响应：

```
[PACKAGE]
NAME = "FILE_ADAPT"
VERSION = "0.1.0"
EDITION = "2024"

[DEPENDENCIES]
AXUM = "0.8.6"
...
```

## 📚 学习目标

这个项目适合以下学习目标：

1. **Rust 异步编程基础**
   - `async/await` 语法
   - `#[tokio::main]` 宏的使用
   - 异步函数的编写

2. **文件 I/O 操作**
   - `tokio::fs::File` 异步文件操作
   - `AsyncBufReadExt` trait 的使用
   - 缓冲读取器 `BufReader`

3. **流式处理**
   - `tokio_stream` 流的概念
   - `StreamExt` trait 的方法
   - 流的转换和映射

4. **Web 开发**
   - axum 路由配置
   - HTTP 响应构建
   - 错误处理最佳实践

5. **HTTP 协议**
   - 响应头设置
   - 内容类型配置
   - 状态码处理

## 🏗 项目结构

```
file_adapt/
├── src/
│   └── main.rs          # 主程序文件，包含所有逻辑
├── Cargo.toml           # 项目依赖配置
└── README.md           # 项目文档
```

## 🔧 代码解读

### 核心组件

1. **主函数 (`main`)**
   ```rust
   #[tokio::main]
   async fn main() {
       let app = Router::new().route("/", get(handler));
       let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
       let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
       axum::serve(listener, app).await.unwrap();
   }
   ```

2. **请求处理器 (`handler`)**
   - 异步打开文件
   - 创建缓冲读取器
   - 转换为流并处理
   - 设置 HTTP 响应头

3. **流式处理**
   ```rust
   let stream = tokio_stream::wrappers::LinesStream::new(lines)
       .map(|result| result.map(|line| line.to_uppercase() + "\n"));
   ```

### 关键概念

- **异步 vs 同步**: 异步操作不会阻塞线程，提高并发性能
- **流式处理**: 逐行处理而非一次性读取整个文件，节省内存
- **错误处理**: 使用 `Result` 类型优雅处理各种错误情况

## 🎯 性能特点

- **内存效率**: 流式处理意味着不需要将整个文件加载到内存
- **并发友好**: 使用异步 I/O，可以同时处理多个请求
- **低延迟**: 逐行处理，客户端可以更快开始接收数据

## 🧪 测试建议

1. **功能测试**
   ```bash
   # 测试正常请求
   curl -f http://localhost:3000

   # 测试文件不存在的情况（重命名 Cargo.toml）
   mv Cargo.toml Cargo.toml.bak
   curl -i http://localhost:3000
   ```

2. **性能测试**
   ```bash
   # 使用 time 命令测量响应时间
   time curl http://localhost:3000 > /dev/null

   # 使用 wrk 进行压力测试
   wrk -t4 -c100 -d30s http://localhost:3000
   ```

3. **大文件测试**
   ```bash
   # 创建大文件测试
   echo "Large file content..." > large_file.txt
   # 修改代码读取 large_file.txt 而不是 Cargo.toml
   ```

## 🔄 扩展建议

可以基于这个项目进行以下扩展：

1. **多文件支持**
   - 添加路径参数支持多个文件
   - 文件列表浏览功能

2. **文件格式转换**
   - 支持 JSON、XML 等格式转换
   - 添加压缩功能

3. **安全增强**
   - 文件访问权限检查
   - 路径遍历攻击防护

4. **配置化**
   - 从配置文件读取设置
   - 命令行参数支持

5. **监控和日志**
   - 添加请求日志
   - 性能指标收集

## 📖 相关资源

### 官方文档
- [axum 官方文档](https://docs.rs/axum/)
- [tokio 官方文档](https://tokio.rs/tokio/tutorial)
- [Rust 异步编程指南](https://rust-lang.github.io/async-book/)
- [Rust 语言指南](https://doc.rust-lang.org/book/)

### 教程和文章
- [Rust Web 开发教程](https://github.com/rust-lang/wg-async-foundations)
- [Tokio 异步编程教程](https://tokio.rs/tokio/tutorial)
- [HTTP 协议基础](https://developer.mozilla.org/zh-CN/docs/Web/HTTP)

### 社区资源
- [Rust 中文社区](https://rust.cc/)
- [Tokio 用户论坛](https://users.rust-lang.org/)
- [Stack Overflow Rust 标签](https://stackoverflow.com/questions/tagged/rust)

## 🤝 贡献指南

欢迎对这个项目提出改进建议！

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [Tokio 团队](https://tokio.rs/) 提供的优秀异步运行时
- [Axum 团队](https://github.com/tokio-rs/axum) 开发的现代化 Web 框架
- [Rust 社区](https://www.rust-lang.org/community) 提供的强大工具和生态

---

**⭐ 如果这个项目对你学习 Rust 异步编程有帮助，请给个 Star！**