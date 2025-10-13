// 数据收集器模块 - 处理来自收集器的 TCP 连接
//
// 这个模块实现了服务器的 TCP 数据接收功能：
// 1. 监听指定端口接受收集器连接
// 2. 异步处理多个并发连接
// 3. 解码和验证收到的数据
// 4. 将数据存储到 SQLite 数据库
//
// 技术特点：
// - 异步 I/O 处理，支持高并发
// - 协议解析和数据验证
// - 数据库事务处理
// - 错误处理和日志记录
//
// 相关文档：
// - Tokio 网络：https://tokio.rs/tokio/tutorial/serving-traffic
// - TCP 协议：https://tools.ietf.org/html/rfc793
// - 异步 Rust：https://rust-lang.github.io/async-book/

use shared_data::{
    CollectorCommandV1, CollectorResponseV1, DATA_COLLECTOR_ADDRESS, decode_v1, encode_response_v1,
};
use sqlx::{Pool, Sqlite}; // 数据库连接池和 SQLite 类型
use std::net::SocketAddr; // 网络地址类型
use tokio::io::AsyncWriteExt;
use tokio::{
    io::AsyncReadExt,              // 异步读取特征
    net::{TcpListener, TcpStream}, // TCP 监听器和流
};

/// 数据收集器主函数 - 启动 TCP 服务器接收收集器数据
///
/// 这个函数是 TCP 数据接收服务的入口点，它：
/// 1. 绑定到指定端口监听 TCP 连接
/// 2. 在无限循环中接受新的连接
/// 3. 为每个连接启动独立的异步任务处理
///
/// # 异步并发处理
/// - 每个新连接都在独立的任务中处理
/// - 使用 tokio::spawn 实现真正的并发
/// - 连接池的克隆允许多个任务共享数据库连接
///
/// # 参数
/// * `conn` - SQLite 数据库连接池，用于存储接收到的数据
///
/// # 返回值
/// anyhow::Result<()> - 成功返回 Ok(())，失败返回错误信息
/// 注意：这个函数包含无限循环，正常情况下不会返回
///
/// # 错误处理
/// - TCP 监听失败会返回错误
/// - 单个连接处理失败不会影响其他连接
/// - 数据库错误会在连接处理函数中记录
///
/// # 性能考虑
/// - 每个连接都是独立的异步任务
/// - 内存使用随连接数线性增长
/// - 可以考虑限制最大并发连接数
///
/// 文档参考：
/// - Tokio TCP 监听器：https://tokio.rs/tokio/tutorial/serving-traffic
/// - 异步任务调度：https://tokio.rs/tokio/tutorial/spawning
pub async fn data_collector(conn: Pool<Sqlite>) -> anyhow::Result<()> {
    // 1. 创建 TCP 监听器，绑定到指定地址
    // DATA_COLLECTOR_ADDRESS 在 shared_data 中定义为 "127.0.0.1:9004"
    let listener = TcpListener::bind(DATA_COLLECTOR_ADDRESS).await?;

    // 2. 无限循环接受新连接
    // 每个 accept() 都会等待新的 TCP 连接
    loop {
        // 3. 接受新的 TCP 连接
        // 返回值：(socket, address) - TCP 流和客户端地址
        let (socket, address) = listener.accept().await?;

        // 4. 为每个连接启动独立的异步任务
        // tokio::spawn 创建新的并发任务
        // socket 和 address 的所有权被移动到新任务中
        // conn.clone() 创建连接池的引用，不是克隆连接
        tokio::spawn(new_collection(socket, address, conn.clone()));
    }
}

/// 单个连接处理函数 - 处理来自特定收集器的数据流
///
/// 这个函数负责处理单个收集器的完整生命周期：
/// 1. 接收和解析数据包
/// 2. 验证数据完整性
/// 3. 将数据存储到数据库
/// 4. 处理连接关闭
///
/// # 协议处理
/// - 使用共享的解码函数解析二进制协议
/// - 处理 CRC 校验和数据验证
/// - 支持连续的数据流处理
///
/// # 参数
/// * `socket` - TCP 流，用于与收集器通信
/// * `address` - 客户端的网络地址
/// * `conn` - 数据库连接池
///
/// # 数据流程
/// 1. 异步读取 TCP 流中的数据
/// 2. 解码二进制协议数据
/// 3. 转换 UUID 格式（u128 -> String）
/// 4. 插入数据库并处理错误
/// 5. 循环处理直到连接关闭
///
/// # 错误处理
/// - TCP 读取错误会导致任务结束
/// - 数据库错误会被记录但不会中断连接
/// - 连接正常关闭时返回
///
/// # 安全考虑
/// - 使用参数化查询防止 SQL 注入
/// - 数据类型转换避免溢出
/// - 内存缓冲区大小固定（1024 字节）
async fn new_collection(mut socket: TcpStream, address: SocketAddr, conn: Pool<Sqlite>) {
    // 记录新连接的建立
    println!("new collection from {address:?}");

    // 创建固定大小的缓冲区用于接收数据
    // 1024 字节应该足够处理单个数据包
    let mut buf = vec![0u8; 1024];

    // 主循环：持续处理来自收集器的数据
    loop {
        // 1. 异步读取 TCP 流数据
        // read() 返回读取的字节数，可能会返回 0 表示连接关闭
        let n = socket
            .read(&mut buf)
            .await
            .expect("failed to read data from socket");

        // 2. 检查连接状态
        if n == 0 {
            println!("No data received - connection closed!");
            return; // 连接已关闭，退出处理函数
        }

        // 3. 记录接收到的数据量
        println!(" received {} bytes", n);

        // 4. 解码二进制协议数据
        // decode_v1 会验证魔数、版本号和 CRC 校验和
        let received_data = decode_v1(&buf[..n]);

        // 5. 处理解码后的数据
        match received_data {
            (
                timestamp, // 数据时间戳
                CollectorCommandV1::SubmitData {
                    // 数据提交命令
                    collector_id,      // 收集器 ID (u128)
                    total_memory,      // 总内存 (u64)
                    used_memory,       // 已用内存 (u64)
                    average_cpu_usage, // 平均 CPU 使用率 (f32)
                },
            ) => {
                // 6. 转换 UUID 格式：u128 -> UUID -> String
                let collector_id = uuid::Uuid::from_u128(collector_id);
                let collector_id = collector_id.to_string();

                // 7. 执行数据库插入操作
                // 使用参数化查询防止 SQL 注入
                let result = sqlx::query(
                    "insert into timeseries(collector_id, received, total_memory, used_memory, average_cpu) values(?, ?, ?, ?, ?)"
                ).bind(collector_id)                    // 绑定收集器 ID
                    .bind(timestamp)                     // 绑定时间戳
                    .bind(total_memory as i64)           // 类型转换：u64 -> i64
                    .bind(used_memory as i64)            // 类型转换：u64 -> i64
                    .bind(average_cpu_usage)             // 绑定 CPU 使用率
                    .execute(&conn)                      // 执行查询
                    .await; // 异步等待结果

                // 8. 处理数据库操作错误
                if result.is_err() {
                    println!("Error insert into the database {result:?}");
                    // 注意：这里不 return，继续处理后续数据包
                } else {
                    let ack = CollectorResponseV1::Ack(0);
                    let response_bytes = encode_response_v1(ack);
                    socket.write_all(&response_bytes).await.unwrap();
                }
            }
        }
    }
}
