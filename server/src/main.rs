// 数据收集服务器主程序
//
// 这个程序实现了分布式监控系统的服务端，具有以下核心功能：
// 1. 异步 TCP 服务器接收收集器数据
// 2. REST API 提供数据查询接口
// 3. SQLite 数据库数据持久化
// 4. 多任务并发处理
//
// 技术栈：
// - Tokio: 异步运行时
// - Axum: Web 框架
// - SQLx: 异步数据库操作
// - Serde: JSON 序列化
//
// 相关文档：
// - Axum 教程：https://docs.rs/axum/latest/axum/
// - Tokio 指南：https://tokio.rs/tokio/tutorial
// - SQLx 文档：https://docs.rs/sqlx/latest/sqlx/
// - 异步 Rust：https://rust-lang.github.io/async-book/

mod collector;

use axum::{Extension, Json, Router, extract::Path, routing::get};
use futures::TryStreamExt;  // 异步流处理扩展
use serde::Serialize;       // JSON 序列化特征
use sqlx;                   // 异步 SQL 数据库操作

/// 异步主函数 - 服务器的入口点
///
/// 这个函数设置了整个服务器的运行环境：
/// 1. 加载环境变量配置
/// 2. 建立数据库连接池
/// 3. 启动后台数据收集任务
/// 4. 配置和启动 Web API 服务器
/// 5. 处理任务生命周期
///
/// # 异步编程概念
/// - #[tokio::main]: 将同步的 main 函数转换为异步运行时
/// - anyhow::Result<>: 统一的错误处理类型
/// - 连接池: 复用数据库连接提高性能
/// - 并发任务: 同时处理 TCP 连接和 HTTP 请求
///
/// # 返回值
/// anyhow::Result<()> - 成功返回 Ok(())，失败返回错误信息
///
/// # 相关概念
/// - 连接池: 数据库连接的复用机制
/// - 异步任务: 并发执行的计算单元
/// - 路由: HTTP 请求到处理函数的映射
///
/// 文档参考：
/// - Tokio runtime: https://tokio.rs/tokio/tutorial/spawning
/// - 连接池模式: https://en.wikipedia.org/wiki/Connection_pool
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 加载 .env 文件中的环境变量
    // dotenv 会从当前目录查找 .env 文件并加载其中的变量
    dotenv::dotenv()?;

    // 2. 从环境变量获取数据库连接字符串
    // 通常格式为："sqlite:collection.db"
    let db_url = std::env::var("DATABASE_URL")?;

    // 3. 创建 SQLite 数据库连接池
    // 连接池管理多个数据库连接，避免频繁创建/销毁连接的开销
    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    // 4. 启动后台数据收集任务
    // tokio::spawn 创建一个新的异步任务，用于处理收集器的 TCP 连接
    // pool.clone() 创建连接池的引用，所有任务共享同一个连接池
    let handle = tokio::spawn(collector::data_collector(pool.clone()));

    // 5. 配置 Axum Web 路由器
    // 定义 API 端点及其对应的处理函数
    let app = Router::new()
        .route("/api/all", get(show_all))                    // 获取所有数据点
        .route("/api/collectors", get(show_collectors))     // 获取收集器列表
        .route("/api/collector/{uuid}", get(collector_data)) // 获取特定收集器数据
        .layer(Extension(pool));                            // 添加数据库连接池作为中间件

    // 6. 创建 TCP 监听器，监听 HTTP 请求
    // 127.0.0.1:3080 是服务器监听的地址和端口
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3080")
        .await
        .unwrap();

    // 7. 启动 Web 服务器
    // axum::serve 会持续监听和处理 HTTP 请求直到程序被终止
    axum::serve(listener, app).await.unwrap();

    // 8. 等待数据收集任务完成
    // 这行代码通常不会执行，因为 Web 服务器会一直运行
    // ?? 用于传播 anyhow 错误
    handle.await??;

    Ok(())
}

use sqlx::{FromRow, Pool};  // SQLx 的数据库行映射和连接池类型

/// 数据点结构体 - 表示单次收集的性能数据
///
/// 这个结构体映射到数据库中的 timeseries 表，包含了：
/// - 系统性能指标（内存、CPU）
/// - 收集器标识和时间戳
/// - 数据库主键
///
/// # 派生特征
/// - FromRow: 允许从数据库行自动映射到此结构体
/// - Debug: 支持格式化输出用于调试
/// - Serialize: 支持序列化为 JSON 用于 API 响应
///
/// # 字段说明
/// - id: 数据库自增主键
/// - collector_id: 收集器的 UUID 字符串
/// - received: 数据接收时间戳（Unix 时间）
/// - total_memory: 系统总内存（字节）
/// - used_memory: 已使用内存（字节）
/// - average_cpu: 平均 CPU 使用率（0.0-1.0）
#[derive(FromRow, Debug, Serialize)]
pub struct DataPoint {
    id: i32,                 // 数据库主键
    collector_id: String,    // 收集器唯一标识符
    received: i64,          // 数据接收时间戳
    total_memory: i64,      // 总内存量（字节）
    used_memory: i64,       // 已使用内存（字节）
    average_cpu: f32,       // 平均 CPU 使用率
}

/// 收集器摘要结构体 - 表示收集器的基本信息和状态
///
/// 这个结构体用于展示收集器列表，包含每个收集器的：
/// - 基本标识信息
/// - 最后活动时间
///
/// # SQL 查询逻辑
/// 通过子查询获取每个收集器的最新活动时间：
/// ```sql
/// SELECT MAX(received) FROM timeseries WHERE collector_id = ts.collector_id
/// ```
///
/// # 字段说明
/// - id: 收集器在数据库中的 ID（实际上可能是第一个数据点的 ID）
/// - collector_id: 收集器的 UUID 字符串
/// - last_seen: 最后一次接收数据的时间戳
#[derive(FromRow, Debug, Serialize)]
pub struct Collector {
    id: i32,              // 数据库 ID（来自第一个数据点）
    collector_id: String, // 收集器唯一标识符
    last_seen: i64,      // 最后活动时间戳
}

/// API 处理函数 - 获取所有数据点
///
/// 这个函数处理 GET /api/all 请求，返回数据库中所有的性能数据点。
/// 通常用于数据分析、图表生成或数据导出等场景。
///
/// # 参数
/// * `Extension(pool)` - 从 Axum 中间件获取的数据库连接池
///
/// # 返回值
/// Json<Vec<DataPoint>> - 所有数据点的 JSON 数组
///
/// # 性能考虑
/// - 随着数据量增长，这个查询可能变得很慢
/// - 在生产环境中应该考虑分页或时间范围过滤
/// - fetch_all() 会将所有数据加载到内存中
///
/// # 错误处理
/// 当前使用 unwrap() 简化错误处理，在生产环境中应该：
/// - 返回合适的 HTTP 错误状态码
/// - 记录详细的错误日志
/// - 提供用户友好的错误信息
///
/// 文档参考：
/// - Axum Extractors: https://docs.rs/axum/latest/axum/extract/index.html
/// - SQLx 查询: https://docs.rs/sqlx/latest/sqlx/fn.query_as.html
async fn show_all(Extension(pool): Extension<Pool<sqlx::Sqlite>>) -> Json<Vec<DataPoint>> {
    // 执行 SQL 查询获取所有数据点
    // SQLx 会自动将数据库行映射到 DataPoint 结构体
    let rows = sqlx::query_as::<_, DataPoint>("select * from timeseries")
        .fetch_all(&pool)  // 获取所有查询结果
        .await
        .unwrap();         // 简化错误处理（生产环境中应改进）

    // 将结果序列化为 JSON 并返回
    // Axum 会自动处理 Content-Type 头设置
    Json(rows)

    // 注释掉的代码展示了流式处理的方式：
    // let mut rows = sqlx::query_as::<_, DataPoint>("select * from timeseries").fetch(&pool);
    // while let Some(row) = rows.try_next().await.unwrap() {
    //     println!("{:?}", row);
    // }
    // 流式处理适合大量数据，可以减少内存使用
}
/// API 处理函数 - 获取收集器列表
///
/// 这个函数处理 GET /api/collectors 请求，返回所有活跃收集器的列表。
/// 通过复杂的 SQL 查询获取每个收集器的唯一标识和最后活动时间。
///
/// # SQL 查询分析
/// 查询使用了子查询来获取每个收集器的最新活动时间：
/// ```sql
/// SELECT
///     DISTINCT(id) AS id,                    -- 去重的 ID
///     collector_id,                          -- 收集器 UUID
///     (SELECT MAX(received)                  -- 子查询：获取最新时间戳
///      FROM timeseries
///      WHERE collector_id = ts.collector_id) AS last_seen
/// FROM timeseries ts                         -- 主表
/// ```
///
/// # 参数
/// * `Extension(pool)` - 数据库连接池
///
/// # 返回值
/// Json<Vec<Collector>> - 收集器列表的 JSON 数组
///
/// # 应用场景
/// - 监控面板显示活跃收集器
/// - 收集器状态管理
/// - 故障检测和告警
///
/// # 性能优化
/// - DISTINCT 减少重复数据
/// - 子查询比 JOIN 更简洁
/// - 可以考虑添加索引优化查询性能
async fn show_collectors(Extension(pool): Extension<Pool<sqlx::Sqlite>>) -> Json<Vec<Collector>> {
    // 复杂的 SQL 查询，获取每个收集器的摘要信息
    const SQL: &str = "SELECT
        DISTINCT(id) AS id,                    -- 去重的收集器 ID
        collector_id,                          -- 收集器唯一标识符
        (SELECT MAX(received)                  -- 子查询：获取最后活动时间
         FROM timeseries
         WHERE collector_id = ts.collector_id) AS last_seen
        FROM timeseries ts                     -- 主查询表
    ";

    // 执行查询并返回结果
    Json(
        sqlx::query_as::<_, Collector>(SQL)
            .fetch_all(&pool)
            .await
            .unwrap(),
    )
}

/// API 处理函数 - 获取特定收集器的数据
///
/// 这个函数处理 GET /api/collector/{uuid} 请求，返回指定收集器的所有历史数据。
/// 使用路径参数获取收集器 UUID，并按时间顺序返回其性能数据。
///
/// # 路径参数
/// * `uuid: Path<String>` - 从 URL 路径中提取的收集器 UUID
///   例如：GET /api/collector/550e8400-e29b-41d4-a716-446655440000
///
/// # 参数
/// * `Extension(pool)` - 数据库连接池
/// * `uuid` - 收集器的 UUID 字符串
///
/// # 返回值
/// Json<Vec<DataPoint>> - 指定收集器的所有数据点，按时间排序
///
/// # SQL 查询
/// ```sql
/// SELECT * FROM timeseries
/// WHERE collector_id = ?
/// ORDER BY received
/// ```
/// 使用参数化查询防止 SQL 注入攻击
///
/// # 安全考虑
/// - 使用参数化查询防止 SQL 注入
/// - UUID 格式验证（可以在路由层面进行）
/// - 结果集大小限制（防止大量数据查询）
///
/// # 应用场景
/// - 特定收集器的详细监控
/// - 历史数据分析和趋势预测
/// - 性能问题诊断
///
/// 文档参考：
/// - Axum 路径参数：https://docs.rs/axum/latest/axum/extract/struct.Path.html
/// - SQLx 参数绑定：https://docs.rs/sqlx/latest/sqlx/trait.Encode.html
async fn collector_data(
    Extension(pool): Extension<Pool<sqlx::Sqlite>>,  // 数据库连接池
    uuid: Path<String>,                              // 从 URL 路径提取的 UUID
) -> Json<Vec<DataPoint>> {
    // 执行参数化查询，获取特定收集器的数据
    // ? 是参数占位符，会被 bind() 方法安全地替换
    let rows = sqlx::query_as::<_, DataPoint>(
        "select * from timeseries where collector_id = ? order by received",
    )
    .bind(uuid.as_str())  // 安全地绑定参数，防止 SQL 注入
    .fetch_all(&pool)
    .await
    .unwrap();

    // 返回按时间排序的数据点
    Json(rows)
}
