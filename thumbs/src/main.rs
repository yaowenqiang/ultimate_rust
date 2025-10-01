//! Thumbs - 简单的图片上传服务器
//!
//! 这是一个基于 Rust 和 Axum 框架的简单图片上传服务器，支持：
//! - 图片上传和标签管理
//! - SQLite 数据库存储元数据
//! - 文件系统存储图片文件
//! - 简单的 Web 界面
//!
//! 主要技术栈：
//! - Axum: 高性能的 Web 框架 (https://github.com/tokio-rs/axum)
//! - SQLx: 异步 SQL 工具包 (https://github.com/launchbadge/sqlx)
//! - Tokio: 异步运行时 (https://tokio.rs/)
//! - Image: 图片处理库 (https://github.com/image-rs/image)
//!
//! API 端点：
//! - GET /: 主页，显示上传表单
//! - POST /upload: 上传图片和标签
//! - GET /image/{id}: 获取指定ID的图片

use axum::extract::{Multipart, Path};
use axum::http::header;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::{Extension, Router};
use futures::{StreamExt, TryStreamExt};
use sqlx::Row;
use std::fmt::format;
use tokio::task::spawn_blocking;
use tokio_util::io::ReaderStream;

/// 程序入口点
///
/// # 返回值
/// - `anyhow::Result<()>`: 成功时返回 Ok(())，错误时返回具体的错误信息
///
/// # 主要流程
/// 1. 加载环境变量配置文件
/// 2. 连接到 SQLite 数据库
/// 3. 运行数据库迁移
/// 4. 配置和启动 HTTP 服务器
///
/// # 参考
/// - Tokio main 函数宏: https://tokio.rs/tokio/tutorial/spawning
/// - Anyhow 错误处理: https://github.com/dtolnay/anyhow
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 从 .env 文件加载环境变量
    // dotenv 用于管理环境变量，避免硬编码配置信息
    // 参考: https://github.com/dotenv-rs/dotenv
    dotenv::dotenv()?;

    // 2. 获取数据库连接 URL
    // 从环境变量中读取数据库连接字符串
    let db_url = std::env::var("DATABASE_URL")?;

    // 3. 创建数据库连接池
    // SQLx 提供了连接池功能，可以复用数据库连接
    // 参考: https://docs.rs/sqlx/latest/sqlx/pool/index.html
    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    // 4. 运行数据库迁移
    // 自动执行 migrations 目录中的 SQL 文件，确保数据库结构是最新的
    // 参考: https://docs.rs/sqlx/latest/sqlx/macro.migrate.html
    sqlx::migrate!("./migrations").run(&pool).await?;

    // 5. 配置 Axum 路由
    // 创建路由器并注册各个处理函数
    // Router 是 Axum 的核心组件，用于组织 HTTP 路由
    // 参考: https://docs.rs/axum/latest/axum/struct.Router.html
    let app = Router::new()
        .route("/", get(index_page)) // 主页
        .route("/upload", post(uploader)) // 图片上传端点
        .route("/image/{id}", get(get_image)) // 图片获取端点
        .layer(Extension(pool)); // 注入数据库连接池

    // 6. 创建 TCP 监听器
    // 绑定到本地地址和端口
    // 参考: https://docs.rs/tokio/latest/tokio/net/struct.TcpListener.html
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8280")
        .await
        .unwrap();

    // 7. 启动 HTTP 服务器
    // axum::serve 会处理 HTTP 请求并分发到对应的路由处理器
    // 参考: https://docs.rs/axum/latest/axum/fn.serve.html
    println!("服务器启动在 http://127.0.0.1:8280");
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

/// 测试数据库连接的函数
///
/// # 参数
/// - `Extension(pool)`: 从 Axum 的 Extension 提取器中获取的数据库连接池
///
/// # 返回值
/// - `String`: 包含图片数量的字符串
///
/// # 功能
/// 查询数据库中图片的总数量，用于验证数据库连接是否正常
///
/// # 注意
/// 这个函数目前没有被使用，可以作为健康检查端点使用
async fn test(Extension(pool): Extension<sqlx::Pool<sqlx::Sqlite>>) -> String {
    // 执行 SQL 查询，统计图片表中的记录数
    // sqlx::query 用于创建 SQL 查询语句
    // 参考: https://docs.rs/sqlx/latest/sqlx/fn.query.html
    let result = sqlx::query("SELECT count(id) FROM images")
        .fetch_one(&pool) // 获取单个结果
        .await
        .unwrap();

    // 从查询结果中提取计数值
    // Row::get 方法用于从数据库行中提取特定列的值
    // 参考: https://docs.rs/sqlx/latest/sqlx/trait.Row.html
    let count = result.get::<i64, _>(0);
    format!("{count} image(s) in the database")
}

/// 主页处理器
///
/// # 返回值
/// - `Html<String>`: 包装在 HTML 响应中的页面内容
///
/// # 功能
/// 读取并返回 index.html 文件的内容作为主页
///
/// # 错误处理
/// 如果文件不存在或读取失败，会 panic
/// 在生产环境中应该使用更优雅的错误处理
///
/// # 参考
/// - Axum HTML 响应: https://docs.rs/axum/latest/axum/response/struct.Html.html
/// - Tokio 文件系统操作: https://tokio.rs/tokio/fs
async fn index_page() -> Html<String> {
    // 构建静态文件路径
    let path = std::path::Path::new("src/index.html");

    // 异步读取文件内容
    // tokio::fs::read_to_string 提供了异步文件读取功能
    // 参考: https://docs.rs/tokio/latest/tokio/fs/fn.read_to_string.html
    let content = tokio::fs::read_to_string(path).await.unwrap();

    // 将字符串包装为 HTML 响应
    // Html 类型会自动设置正确的 Content-Type 头
    Html(content)
}

/// 图片上传处理器
///
/// # 参数
/// - `Extension(pool)`: 数据库连接池
/// - `multipart`: 多部分表单数据，包含上传的文件和标签
///
/// # 返回值
/// - `String`: 上传结果的简单响应
///
/// # 功能
/// 1. 解析多部分表单数据
/// 2. 提取标签和图片数据
/// 3. 将图片元数据保存到数据库
/// 4. 将图片文件保存到文件系统
///
/// # 表单字段
/// - `tags`: 图片的标签（字符串）
/// - `image`: 图片文件（二进制数据）
///
/// # 错误处理
/// 如果缺少必需的字段，会 panic
/// 在生产环境中应该返回适当的 HTTP 状态码
///
/// # 参考
/// - Axum 多部分表单处理: https://docs.rs/axum/latest/axum/extract/struct.Multipart.html
async fn uploader(
    Extension(pool): Extension<sqlx::Pool<sqlx::Sqlite>>,
    mut multipart: Multipart,
) -> String {
    // 初始化变量来存储上传的数据
    let mut tags = None; // 存储标签字符串
    let mut image = None; // 存储图片二进制数据

    // 遍历多部分表单的所有字段
    // multipart.next_field() 返回表单中的下一个字段
    // 参考: https://docs.rs/axum/latest/axum/extract/struct.Multipart.html#method.next_field
    while let Some(field) = multipart.next_field().await.unwrap() {
        // 获取字段名称
        let name = field.name().unwrap().to_string();

        // 获取字段数据（字节格式）
        // field.bytes() 读取字段的二进制内容
        let data = field.bytes().await.unwrap();

        // 根据字段名称分类处理
        match name.as_str() {
            "tags" => {
                // 将标签数据转换为字符串
                tags = Some(String::from_utf8(data.to_vec()).unwrap());
            }
            "image" => {
                // 直接存储图片的二进制数据
                image = Some(data.to_vec());
            }
            _ => {
                // 遇到未知字段时 panic
                panic!("unknown field {name}");
            }
        }
    }

    // 检查是否同时收到了标签和图片
    if let (Some(tags), Some(image)) = (tags, image) {
        // 1. 先在数据库中插入图片记录
        let new_image_id = insert_image_into_database(&pool, &tags).await.unwrap();

        // 2. 然后将图片文件保存到磁盘
        save_image(new_image_id, &image).await.unwrap();
    } else {
        // 如果缺少必需字段则 panic
        panic!("missing field");
    }

    // 返回成功响应
    "OK".to_string()
}

/// 将图片元数据插入数据库
///
/// # 参数
/// - `pool`: 数据库连接池的引用
/// - `tags`: 图片标签字符串
///
/// # 返回值
/// - `anyhow::Result<i64>`: 新插入图片的 ID，或错误信息
///
/// # 功能
/// 在 images 表中插入新记录，并返回自动生成的 ID
///
/// # SQL 语句
/// `INSERT INTO images(tags) values (?) RETURNING id`
/// - 插入标签数据
/// - RETURNING id 获取新插入记录的 ID
///
/// # 参考
/// - SQLx 参数绑定: https://docs.rs/sqlx/latest/sqlx/fn.query.html#binding-parameters
/// - SQLite RETURNING 子句: https://www.sqlite.org/lang_returning.html
async fn insert_image_into_database(
    pool: &sqlx::Pool<sqlx::Sqlite>,
    tags: &str,
) -> anyhow::Result<i64> {
    // 创建带参数的 SQL 查询
    // 使用 ? 作为占位符可以防止 SQL 注入
    // 参考: https://docs.rs/sqlx/latest/sqlx/fn.query.html
    let row = sqlx::query("INSERT INTO images(tags) values (?) RETURNING id")
        .bind(tags) // 绑定参数，自动处理类型转换和安全性
        .fetch_one(pool) // 期望返回单行结果
        .await?;

    // 从返回的行中提取 ID（第一列）
    Ok(row.get(0))
}

/// 将图片文件保存到磁盘
///
/// # 参数
/// - `id`: 图片的数据库 ID
/// - `bytes`: 图片的二进制数据
///
/// # 返回值
/// - `anyhow::Result<()>`: 成功时返回 Ok(())，失败时返回错误信息
///
/// # 功能
/// 1. 创建图片存储目录（如果不存在）
/// 2. 检查文件是否已存在（防止覆盖）
/// 3. 将图片数据写入文件系统
///
/// # 文件命名规则
/// 图片文件名为 `{id}.png`，存储在 `image/` 目录下
///
/// # 安全考虑
/// 检查文件是否已存在，防止意外覆盖已有文件
///
/// # 参考
/// - Tokio 文件系统操作: https://tokio.rs/tokio/fs
/// - Rust 路径处理: https://doc.rust-lang.org/std/path/index.html
async fn save_image(id: i64, bytes: &[u8]) -> anyhow::Result<()> {
    // 定义图片存储的基础目录
    let base_path = std::path::Path::new("image");

    // 检查目录是否存在，如果不存在则创建
    // create_dir_all 会递归创建所有必要的父目录
    if !base_path.exists() || !base_path.is_dir() {
        tokio::fs::create_dir_all(base_path).await?;
    }

    // 构建图片文件的完整路径
    // 使用 ID 作为文件名，扩展名为 .png
    let image_path = base_path.join(format!("{id}.png"));

    // 检查文件是否已存在，防止覆盖
    if image_path.exists() {
        anyhow::bail!("File already exists");
    }

    // 将图片数据写入文件
    // tokio::fs::write 会原子性地写入整个文件
    // 参考: https://docs.rs/tokio/latest/tokio/fs/fn.write.html
    tokio::fs::write(image_path, bytes).await?;

    Ok(())
}

/// 图片获取处理器
///
/// # 参数
/// - `Path(id)`: 从 URL 路径中提取的图片 ID
///
/// # 返回值
/// - `impl IntoResponse`: HTTP 响应，包含图片数据
///
/// # 功能
/// 1. 根据构建文件路径
/// 2. 打开图片文件
/// 3. 将文件内容作为流式 HTTP 响应返回
///
/// # HTTP 响应特性
/// - Content-Type: image/png
/// - 流式传输，适合大文件
///
/// # 错误处理
/// 如果文件不存在，会 panic
/// 在生产环境中应该返回 404 Not Found
///
/// # 参考
/// - Axum 路径参数提取: https://docs.rs/axum/latest/axum/extract/struct.Path.html
/// - Tokio 文件流: https://docs.rs/tokio-util/latest/tokio_util/io/struct.ReaderStream.html
/// - Axum 响应构建: https://docs.rs/axum/latest/axum/response/index.html
async fn get_image(Path(id): Path<i64>) -> impl IntoResponse {
    // 根据构建图片文件路径
    let filename = format!("image/{id}.png");

    // 异步打开文件
    // tokio::fs::File 提供了异步文件读取功能
    // 参考: https://docs.rs/tokio/latest/tokio/fs/struct.File.html
    let file = tokio::fs::File::open(filename).await.unwrap();

    // 将文件转换为异步流
    // ReaderStream 将实现了 AsyncRead 的类型转换为 Stream
    // 这样可以实现流式响应，避免大文件占用过多内存
    // 参考: https://docs.rs/tokio-util/latest/tokio_util/io/struct.ReaderStream.html
    let stream = ReaderStream::new(file);

    // 将流转换为 Axum 的 Body 类型
    // Body::from_stream 将异步流转换为 HTTP 响应体
    let body = axum::body::Body::from_stream(stream);

    // 构建自定义 HTTP 响应
    // Response::builder 提供了灵活的响应构建方式
    axum::response::Response::builder()
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("image/png"),
        )
        .body(body) // 使用流式 body
        .unwrap()
}
fn make_thumbnail(id: i64) -> anyhow::Result<()> {
    let image_path = format!("image/{id}.jpg");
    let thumbnail_path = format!("image/{id}_thumb.jpg");
    let image_bytes = std::fs::read(image_path)?;
    let image = if let Ok(format) = image::guess_format(&image_bytes) {
        image::load_from_memory_with_format(&image_bytes, format)?
    } else {
        image::load_from_memory(&image_bytes)?
    };
    let thumbnail = image.thumbnail(100, 100);
    thumbnail.save(thumbnail_path)?;
    Ok(())
}

async fn fill_missing_thumbnails(pool: sqlx::Pool<sqlx::Sqlite>) -> anyhow::Result<()> {
    let mut rows = sqlx::query("select id from images").fetch(&pool);

    while let Some(row) = rows.try_next().await? {
        let id = row.get::<i64, _>(0);
        let thumbnail_path = format!("images/{id}_thumb.jpg");
        if !std::path::Path::new(&thumbnail_path).exists() {
            spawn_blocking(move || make_thumbnail(id)).await??;
        }
    }

    Ok(())
}
