//! file_adapt - Rust 异步流式文件服务器
//!
//! 这是一个使用 axum 框架构建的简单 Web 服务器，演示了异步文件读取和流式响应的概念。
//! 该服务器会读取当前目录下的 Cargo.toml 文件，将其内容转换为大写字母，
//! 并以流式方式返回给客户端。
//!
//! ## 主要特性
//! - 异步文件读取，不阻塞线程
//! - 流式响应，适合大文件传输
//! - 文件内容实时转换（大写转换）
//! - 现代 Rust 异步编程实践
//!
//! ## 技术栈
//! - axum: 现代化的 Rust Web 框架
//! - tokio: 异步运行时
//! - tokio-stream: 异步流处理
//!
//! ## 相关文档
//! - [axum 官方文档](https://docs.rs/axum/)
//! - [tokio 官方文档](https://docs.rs/tokio/)
//! - [Rust 异步编程指南](https://rust-lang.github.io/async-book/)
//! - [Rust Web 开发最佳实践](https://github.com/rust-lang/wg-async-foundations)

use axum::{
    body::Body,                                   // HTTP 响应体类型
    http::{HeaderMap, header, StatusCode},       // HTTP 相关类型
    response::IntoResponse,                      // 响应转换 trait
    routing::get,                                // 路由方法
    Router,                                      // 路由器类型
};
use tokio::io::{AsyncBufReadExt, BufReader};     // 异步缓冲读取
use std::net::SocketAddr;                        // 网络地址类型
use axum::http::HeaderValue;                     // HTTP 头部值类型
use tokio_stream::StreamExt;                     // 流处理扩展 trait
use std::str::FromStr;                           // 字符串解析 trait

/// 主函数 - 程序入口点
///
/// 初始化并启动 axum Web 服务器：
/// 1. 创建路由器，配置根路径 "/" 的处理器
/// 2. 绑定到本地地址 127.0.0.1:3000
/// 3. 启动异步服务器
#[tokio::main]
async fn main() {
    // 创建路由器，为根路径配置处理器函数
    let app = Router::new().route("/", get(handler));

    // 配置服务器监听地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // 创建 TCP 监听器
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // 启动服务器并处理请求
    axum::serve(listener, app).await.unwrap();
}

/// HTTP 请求处理器
///
/// 处理对根路径 "/" 的 HTTP GET 请求：
/// 1. 尝试打开并读取 Cargo.toml 文件
/// 2. 如果文件不存在，返回 404 错误
/// 3. 逐行读取文件内容并转换为大写
/// 4. 以流式方式返回处理后的内容
/// 5. 设置适当的 HTTP 响应头
///
/// # Returns
///
/// 返回实现了 `IntoResponse` trait 的对象，包含：
/// - 文件流数据（大写格式）
/// - 正确的 HTTP 响应头
/// - 错误处理（如果文件不存在）
///
/// # Examples
///
/// 客户端请求示例：
/// ```bash
/// curl http://localhost:3000
/// ```
///
/// 将返回 Cargo.toml 文件内容的大写版本
async fn handler() -> impl IntoResponse {
    // 尝试异步打开 Cargo.toml 文件
    let file = match tokio::fs::File::open("Cargo.toml").await {
        Ok(file) => file,  // 文件打开成功，继续处理
        Err(e) => return (  // 文件打开失败，返回 404 错误
            StatusCode::NOT_FOUND,
            format!("File not found: {}", e)
        ).into_response(),
    };

    // 创建带缓冲的异步文件读取器
    let reader = BufReader::new(file);

    // 获取文件行的异步迭代器
    let lines = reader.lines();

    // 将异步行迭代器转换为流，并应用大写转换
    // 每行读取后都会立即转换为大写并添加换行符
    let stream = tokio_stream::wrappers::LinesStream::new(lines)
        .map(|result| result.map(|line| line.to_uppercase() + "\n"));

    // 创建 HTTP 响应体，基于处理后的流
    let body = Body::from_stream(stream);

    // 配置 HTTP 响应头
    let mut headers = HeaderMap::new();

    // 设置内容类型为纯文本，UTF-8 编码
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=utf-8")
    );

    // 设置内容处置头，提示浏览器将响应作为文件下载
    if let Ok(content_disposition) = HeaderValue::from_str("attachment; filename=\"file.txt\"") {
        headers.insert(header::CONTENT_DISPOSITION, content_disposition);
    }

    // 返回带有头部和流式体的响应
    (headers, body).into_response()
}