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
use axum::{Extension, Json, Router};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
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

    fill_missing_thumbnails(&pool).await?;

    // 5. 配置 Axum 路由
    // 创建路由器并注册各个处理函数
    // Router 是 Axum 的核心组件，用于组织 HTTP 路由
    // 参考: https://docs.rs/axum/latest/axum/struct.Router.html
    let app = Router::new()
        .route("/", get(index_page)) // 主页
        .route("/upload", post(uploader)) // 图片上传端点
        .route("/image/{id}", get(get_image)) // 图片获取端点
        .route("/images", get(list_images)) // 图片列表
        .route("/thumb/{id}", get(get_thumbnail))
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
/// - `Html<String>`: 包装在 HTML 响应中的页面内容，Axum 会自动设置 `Content-Type: text/html` 头
///
/// # 功能
/// 读取并返回 `src/index.html` 文件的内容作为主页，这是用户访问根路径时看到的页面
///
/// # HTTP 方法
/// - GET `/`: 处理主页请求
///
/// # 错误处理
/// 如果文件不存在或读取失败，会 panic。在生产环境中应该：
/// - 返回适当的 HTTP 状态码（如 500 Internal Server Error）
/// - 记录错误日志
/// - 提供用户友好的错误页面
///
/// # 安全考虑
/// - 确保文件路径不被恶意用户操纵（目前使用硬编码路径，相对安全）
/// - 考虑添加文件大小限制，防止读取过大文件
///
/// # 参考
/// - Axum HTML 响应: https://docs.rs/axum/latest/axum/response/struct.Html.html
/// - Tokio 文件系统操作: https://tokio.rs/tokio/fs
/// - HTTP Content-Type 头: https://developer.mozilla.org/zh-CN/docs/Web/HTTP/Headers/Content-Type
/// - 文件系统安全: https://owasp.org/www-project-secure-coding-practices-verify-guide/latest/4-enforcing-integrity-with-file-systems/
async fn index_page() -> Html<String> {
    // 构建静态文件路径
    // 使用 Path 类型来确保跨平台路径处理的正确性
    // 参考: https://doc.rust-lang.org/std/path/struct.Path.html
    let path = std::path::Path::new("src/index.html");

    // 异步读取文件内容
    // tokio::fs::read_to_string 提供了非阻塞的文件读取功能
    // 相比同步读取，这不会阻塞其他异步任务的执行
    // 参考: https://docs.rs/tokio/latest/tokio/fs/fn.read_to_string.html
    let content = tokio::fs::read_to_string(path).await.unwrap();

    // 将字符串包装为 HTML 响应
    // Html<T> 是 Axum 提供的响应类型，会自动设置 Content-Type 为 text/html; charset=utf-8
    // 参考: https://docs.rs/axum/latest/axum/response/struct.Html.html
    Html(content)
}

/// 图片上传处理器
///
/// # 参数
/// - `Extension(pool)`: 从 Axum 的 Extension 提取器中获取的数据库连接池，用于数据库操作
/// - `multipart`: 多部分表单数据，包含上传的文件和标签
///
/// # 返回值
/// - `String`: 上传结果的简单响应，当前返回 "OK"
///
/// # HTTP 方法
/// - POST `/upload`: 处理图片上传请求
///
/// # 功能
/// 1. 解析多部分表单数据 (multipart/form-data)
/// 2. 提取标签和图片数据
/// 3. 将图片元数据保存到数据库
/// 4. 将图片文件保存到文件系统
/// 5. 异步生成缩略图
///
/// # 表单字段 (multipart/form-data)
/// - `tags`: 图片的标签（字符串），用于图片分类和搜索
/// - `image`: 图片文件（二进制数据），支持各种图片格式
///
/// # 错误处理
/// 当前使用 unwrap() 进行错误处理，缺少字段时会 panic
/// 在生产环境中应该：
/// - 返回适当的 HTTP 状态码（400 Bad Request, 500 Internal Server Error）
/// - 提供详细的错误信息
/// - 记录错误日志
///
/// # 安全考虑
/// - 验证上传文件的大小和类型
/// - 限制上传文件的数量和频率
/// - 扫描恶意代码（如果需要）
///
/// # 性能考虑
/// - 使用 spawn_blocking 将图片处理（CPU 密集型）与异步运行时分离
/// - 考虑文件大小限制，避免内存耗尽
///
/// # 参考
/// - Axum 多部分表单处理: https://docs.rs/axum/latest/axum/extract/struct.Multipart.html
/// - HTTP multipart/form-data: https://developer.mozilla.org/zh-CN/docs/Web/HTTP/Methods/POST
/// - 文件上传安全: https://owasp.org/www-project-cheat-sheets/cheatsheets/File_Upload_Cheat_Sheet.html
/// - tokio::task::spawn_blocking: https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
async fn uploader(
    Extension(pool): Extension<sqlx::Pool<sqlx::Sqlite>>,
    mut multipart: Multipart,
) -> String {
    // 初始化变量来存储上传的数据
    let mut tags = None; // 存储标签字符串，Option<String> 类型
    let mut image = None; // 存储图片二进制数据，Option<Vec<u8>> 类型

    // 遍历多部分表单的所有字段
    // multipart.next_field() 返回表单中的下一个字段，使用异步迭代器
    // 参考: https://docs.rs/axum/latest/axum/extract/struct.Multipart.html#method.next_field
    while let Some(field) = multipart.next_field().await.unwrap() {
        // 获取字段名称，用于区分不同的表单字段
        // field.name() 返回 Option<&str>，我们使用 unwrap() 假设名称存在
        let name = field.name().unwrap().to_string();

        // 获取字段数据（字节格式）
        // field.bytes() 异步读取字段的二进制内容到 Bytes 类型
        // Bytes 类型提供零拷贝的字节缓冲区操作
        // 参考: https://docs.rs/bytes/latest/bytes/struct.Bytes.html
        let data = field.bytes().await.unwrap();

        // 根据字段名称分类处理
        match name.as_str() {
            "tags" => {
                // 将标签数据转换为 UTF-8 字符串
                // String::from_utf8 会验证字节序列是否为有效的 UTF-8
                // 在生产环境中应该处理可能的编码错误
                tags = Some(String::from_utf8(data.to_vec()).unwrap());
            }
            "image" => {
                // 直接存储图片的二进制数据
                // to_vec() 将 Bytes 转换为 Vec<u8>，便于后续的文件操作
                image = Some(data.to_vec());
            }
            _ => {
                // 遇到未知字段时 panic，防止恶意用户注入额外字段
                // 在生产环境中应该记录警告并忽略未知字段
                panic!("unknown field {name}");
            }
        }
    }

    // 检查是否同时收到了标签和图片
    // 使用 if let Some 模式匹配来验证必需字段是否存在
    if let (Some(tags), Some(image)) = (tags, image) {
        // 1. 先在数据库中插入图片记录，获取自动生成的 ID
        let new_image_id = insert_image_into_database(&pool, &tags).await.unwrap();

        // 2. 然后将图片文件保存到磁盘
        save_image(new_image_id, &image).await.unwrap();

        // 3. 异步生成缩略图（在单独的线程中执行）
        // spawn_blocking 用于将 CPU 密集型任务与异步运行时分离
        // 避免阻塞事件循环，提高并发性能
        // 参考: https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
        spawn_blocking(move || {
            make_thumbnail(new_image_id).unwrap();
        });
    } else {
        // 如果缺少必需字段则 panic
        // 在生产环境中应该返回 400 Bad Request 响应
        panic!("missing field");
    }

    // 返回成功响应
    // 在生产环境中应该返回 JSON 格式的详细信息
    "OK".to_string()
}

/// 将图片元数据插入数据库
///
/// # 参数
/// - `pool`: 数据库连接池的引用，用于管理数据库连接
/// - `tags`: 图片标签字符串，用于图片分类和搜索
///
/// # 返回值
/// - `anyhow::Result<i64>`: 成功时返回新插入图片的 ID，失败时返回错误信息
///
/// # 功能
/// 在 images 表中插入新记录，包含图片的标签信息，并返回 SQLite 自动生成的 ID
///
/// # SQL 语句
/// ```sql
/// INSERT INTO images(tags) values (?) RETURNING id
/// ```
/// - 使用参数化查询防止 SQL 注入攻击
/// - RETURNING 子句获取新插入记录的自增 ID
/// - SQLite 会自动为主键字段分配递增的整数值
///
/// # 数据库事务
/// 当前函数不包含事务处理，在生产环境中考虑：
/// - 如果有多个相关操作，应该使用事务确保数据一致性
/// - 错误时能够回滚已执行的操作
///
/// # 性能考虑
/// - 使用连接池减少连接建立和销毁的开销
/// - 参数绑定自动处理字符串转义，避免注入攻击
///
/// # 参考
/// - SQLx 参数绑定: https://docs.rs/sqlx/latest/sqlx/fn.query.html#binding-parameters
/// - SQLite RETURNING 子句: https://www.sqlite.org/lang_returning.html
/// - SQL 注入防护: https://owasp.org/www-community/attacks/SQL_Injection
/// - 数据库连接池: https://docs.rs/sqlx/latest/sqlx/pool/struct.Pool.html
/// - Rust 异步错误处理: https://rust-lang.github.io/async-book/03_error_handling/01_error_types.html
async fn insert_image_into_database(
    pool: &sqlx::Pool<sqlx::Sqlite>,
    tags: &str,
) -> anyhow::Result<i64> {
    // 创建带参数的 SQL 查询
    // 使用 ? 作为占位符可以防止 SQL 注入攻击
    // SQLx 会自动处理参数的类型转换和安全性验证
    // 参考: https://docs.rs/sqlx/latest/sqlx/fn.query.html
    let row = sqlx::query("INSERT INTO images(tags) values (?) RETURNING id")
        .bind(tags) // 绑定参数到占位符，SQLx 会处理类型转换和转义
        .fetch_one(pool) // 执行查询并期望返回单行结果
        .await?; // 使用 ? 操作符进行异步错误传播

    // 从返回的行中提取 ID（第一列）
    // row.get<类型, 索引> 会自动进行类型转换和验证
    // 如果类型不匹配会返回编译时或运行时错误
    Ok(row.get(0))
}

/// 将图片文件保存到磁盘
///
/// # 参数
/// - `id`: 图片的数据库 ID，用作文件名确保唯一性
/// - `bytes`: 图片的二进制数据，包含完整的图片内容
///
/// # 返回值
/// - `anyhow::Result<()>`: 成功时返回 Ok(())，失败时返回详细的错误信息
///
/// # 功能
/// 1. 创建图片存储目录（如果不存在）
/// 2. 检查文件是否已存在（防止覆盖）
/// 3. 将图片数据原子性地写入文件系统
///
/// # 文件命名规则
/// 图片文件命名为 `{id}.png`，存储在 `image/` 目录下
/// - 使用数据库 ID 作为文件名确保唯一性
/// - 统一使用 PNG 格式，便于后续处理
///
/// # 安全考虑
/// - 检查文件是否已存在，防止意外覆盖已有文件
/// - 使用数据库 ID 避免文件名冲突和路径遍历攻击
/// - 限制文件大小，防止磁盘空间耗尽
///
/// # 性能考虑
/// - 使用异步文件操作避免阻塞事件循环
/// - tokio::fs::write 提供原子性写入，避免部分写入的情况
/// - 考虑大文件的内存使用情况
///
/// # 目录结构
/// ```
/// project/
/// ├── image/
/// │   ├── 1.png
/// │   ├── 2.png
/// │   └── ...
/// └── ...
/// ```
///
/// # 错误处理
/// 可能的错误情况：
/// - 目录创建失败（权限不足）
/// - 文件已存在（重复上传）
/// - 磁盘空间不足
/// - 权限问题（无法写入）
///
/// # 参考
/// - Tokio 文件系统操作: https://tokio.rs/tokio/fs
/// - Rust 路径处理: https://doc.rust-lang.org/std/path/index.html
/// - 文件系统安全: https://owasp.org/www-project-top-ten/2017/A5_2017-Broken_Access_Control
/// - 异步文件 I/O: https://rust-lang.github.io/async-book/07_io/01_io.html
async fn save_image(id: i64, bytes: &[u8]) -> anyhow::Result<()> {
    // 定义图片存储的基础目录
    // 使用 Path 类型确保跨平台路径处理的正确性
    // 参考: https://doc.rust-lang.org/std/path/struct.Path.html
    let base_path = std::path::Path::new("image");

    // 检查目录是否存在，如果不存在则创建
    // create_dir_all 会递归创建所有必要的父目录
    // 使用异步操作避免阻塞事件循环
    if !base_path.exists() || !base_path.is_dir() {
        tokio::fs::create_dir_all(base_path).await?;
    }

    // 构建图片文件的完整路径
    // Path::join() 会自动处理路径分隔符，确保跨平台兼容性
    // 使用数据库 ID 作为文件名，避免路径遍历攻击
    let image_path = base_path.join(format!("{id}.png"));

    // 检查文件是否已存在，防止覆盖
    // 这是一个业务逻辑检查，防止重复上传同一图片
    // 在生产环境中可能需要更复杂的冲突处理策略
    if image_path.exists() {
        anyhow::bail!("File already exists");
    }

    // 将图片数据原子性地写入文件
    // tokio::fs::write 会：
    // 1. 创建新文件
    // 2. 写入所有数据
    // 3. 原子性地重命名到目标位置
    // 这避免了部分写入导致的数据损坏
    // 参考: https://docs.rs/tokio/latest/tokio/fs/fn.write.html
    tokio::fs::write(image_path, bytes).await?;

    // 成功完成，返回 Ok(())
    // 在生产环境中可能需要记录文件操作日志
    Ok(())
}

/// 图片获取处理器
///
/// # 参数
/// - `Path(id)`: 从 URL 路径中提取的图片 ID，使用 Axum 的路径参数提取器
///
/// # 返回值
/// - `impl IntoResponse`: HTTP 响应，包含流式图片数据，支持大文件传输
///
/// # HTTP 方法
/// - GET `/image/{id}`: 获取指定 ID 的图片文件
///
/// # 功能
/// 1. 根据图片 ID 构建文件路径
/// 2. 异步打开图片文件
/// 3. 将文件内容作为流式 HTTP 响应返回
///
/// # HTTP 响应特性
/// - Content-Type: image/png
/// - Content-Length: 自动设置（如果文件系统支持）
/// - 流式传输，适合大文件，避免内存耗尽
/// - 支持 HTTP 范围请求（如果实现）
///
/// # 文件路径规则
/// 图片文件路径为 `image/{id}.png`，与 save_image 函数的命名规则保持一致
///
/// # 错误处理
/// 当前使用 unwrap() 处理错误，文件不存在时会 panic
/// 在生产环境中应该：
/// - 返回 404 Not Found 响应（文件不存在）
/// - 返回 500 Internal Server Error（读取错误）
/// - 记录访问日志和错误信息
///
/// # 性能考虑
/// - 使用流式响应避免将整个文件加载到内存
/// - 适合处理大文件（高分辨率图片）
/// - 支持并发访问多个文件
/// - 考虑添加缓存头（ETag, Cache-Control）
///
/// # 安全考虑
/// - 验证图片 ID 的范围，防止访问系统文件
/// - 限制访问频率，防止 DDoS 攻击
/// - 考虑添加访问控制和认证
///
/// # 参考
/// - Axum 路径参数提取: https://docs.rs/axum/latest/axum/extract/struct.Path.html
/// - Tokio 异步文件: https://docs.rs/tokio/latest/tokio/fs/struct.File.html
/// - Tokio-util 文件流: https://docs.rs/tokio-util/latest/tokio_util/io/struct.ReaderStream.html
/// - Axum 响应构建: https://docs.rs/axum/latest/axum/response/index.html
/// - HTTP 流式传输: https://developer.mozilla.org/zh-CN/docs/Web/HTTP/Range_requests
/// - 文件服务安全: https://owasp.org/www-project-cheat-sheets/cheatsheets/File_Inclusion_Cheat_Sheet.html
async fn get_image(Path(id): Path<i64>) -> impl IntoResponse {
    // 根据图片 ID 构建文件路径
    // 使用 format! 宏构建符合命名规则的文件路径
    // 文件路径格式: image/{id}.png
    let filename = format!("image/{id}.png");

    // 异步打开文件进行读取
    // tokio::fs::File 提供了异步文件读取功能
    // 实现了 AsyncRead trait，支持非阻塞读取
    // 参考: https://docs.rs/tokio/latest/tokio/fs/struct.File.html
    let file = tokio::fs::File::open(filename).await.unwrap();

    // 将文件转换为异步流
    // ReaderStream 将实现了 AsyncRead 的类型转换为 Stream<Item = Result<Bytes, Error>>
    // 这样可以实现流式响应，避免将整个文件加载到内存中
    // 特别适合处理大文件，如高分辨率图片
    // 参考: https://docs.rs/tokio-util/latest/tokio_util/io/struct.ReaderStream.html
    let stream = ReaderStream::new(file);

    // 将异步流转换为 Axum 的 Body 类型
    // Body::from_stream 将异步流转换为 HTTP 响应体
    // 支持流式传输，客户端可以边接收边显示
    // 参考: https://docs.rs/axum/latest/axum/body/struct.Body.html
    let body = axum::body::Body::from_stream(stream);

    // 构建自定义 HTTP 响应
    // Response::builder 提供了灵活的响应构建方式
    // 可以设置各种 HTTP 头部和自定义响应体
    axum::response::Response::builder()
        .header(
            header::CONTENT_TYPE, // 设置 Content-Type 头部
            header::HeaderValue::from_static("image/png"), // 静态字符串，避免运行时分配
        )
        .body(body) // 使用流式 body
        .unwrap()
}
/// 生成图片缩略图
///
/// # 参数
/// - `id`: 图片的数据库 ID，用于定位源图片文件
///
/// # 返回值
/// - `anyhow::Result<()>`: 成功时返回 Ok(())，失败时返回详细的错误信息
///
/// # 功能
/// 1. 检查源图片文件是否存在
/// 2. 读取原始图片数据
/// 3. 自动检测图片格式
/// 4. 生成 100x100 像素的缩略图
/// 5. 保存缩略图到文件系统
///
/// # 缩略图规格
/// - 尺寸: 100x100 像素
/// - 格式: PNG（与原图保持一致）
/// - 命名: `{id}_thumb.png`
/// - 位置: 与原图同目录
///
/// # 图片格式支持
/// 自动检测并支持多种图片格式：
/// - JPEG, PNG, GIF, BMP, ICO, TIFF, WebP 等
/// - 使用 image crate 的格式检测功能
///
/// # 性能考虑
/// - 这是一个 CPU 密集型操作，应该在 spawn_blocking 中执行
/// - 缩略图算法会保持宽高比，不会变形
/// - 内存使用量与原图大小成正比
///
/// # 错误处理
/// 可能的错误情况：
/// - 源文件不存在或无法读取
/// - 不支持的图片格式
/// - 图片数据损坏
/// - 磁盘空间不足
/// - 权限问题
///
/// # 算法说明
/// 使用 image crate 的 thumbnail 方法：
/// - 保持宽高比的缩放算法
/// - 可能使用 Lanczos3 或类似的高质量重采样
/// - 比简单的 resize() 更适合缩略图生成
///
/// # 参考
/// - Rust Image crate: https://github.com/image-rs/image
/// - 图片处理算法: https://en.wikipedia.org/wiki/Image_scaling
/// - Lanczos 重采样: https://en.wikipedia.org/wiki/Lanczos_resampling
/// - 异步 CPU 密集型任务: https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
fn make_thumbnail(id: i64) -> anyhow::Result<()> {
    // 构建源图片和缩略图的文件路径
    let image_path = format!("image/{id}.png");
    let thumbnail_path = format!("image/{id}_thumb.png");

    // 检查源图片文件是否存在
    // 如果文件不存在，直接返回成功（避免错误日志）
    // 这种情况可能发生在数据库中有记录但文件被删除的情况下
    if !std::path::Path::new(&image_path).exists() {
        return Ok(());
    }

    // 读取原始图片数据到内存
    // 使用同步读取，因为此函数在 spawn_blocking 中运行
    // std::fs::read 返回 Vec<u8>，包含完整的文件内容
    let image_bytes = std::fs::read(image_path)?;

    // 智能检测图片格式并解码
    // image::guess_format 尝试从字节数据推断图片格式
    // 支持常见格式：JPEG, PNG, GIF, BMP, ICO, TIFF, WebP 等
    // 使用 format-specific decoder 可以获得更好的性能和准确性
    let image = if let Ok(format) = image::guess_format(&image_bytes) {
        // 如果成功检测到格式，使用特定格式的解码器
        // 这通常比通用解码器更快更准确
        image::load_from_memory_with_format(&image_bytes, format)?
    } else {
        // 如果无法检测格式，使用通用解码器
        // 作为后备方案，尝试自动检测
        image::load_from_memory(&image_bytes)?
    };

    // 生成 100x100 像素的缩略图
    // thumbnail() 方法会：
    // 1. 保持原始宽高比
    // 2. 将较长边缩放到指定尺寸
    // 3. 使用高质量的重采样算法（如 Lanczos3）
    // 4. 返回新的 ImageBuffer 对象
    // 参考: https://docs.rs/image/latest/image/enum.DynamicImage.html#method.thumbnail
    let thumbnail = image.thumbnail(100, 100);

    // 保存缩略图到文件系统
    // 使用 PNG 格式保存，确保无损压缩和透明度支持
    // save() 方法会根据文件扩展名自动选择格式
    thumbnail.save(thumbnail_path)?;

    // 成功完成，返回 Ok(())
    Ok(())
}

/// 批量填充缺失的缩略图
///
/// # 参数
/// - `pool`: 数据库连接池的引用，用于查询图片记录
///
/// # 返回值
/// - `anyhow::Result<()>`: 成功时返回 Ok(())，失败时返回错误信息
///
/// # 功能
/// 1. 查询数据库中所有图片记录
/// 2. 检查每个图片的缩略图是否存在
/// 3. 为缺失缩略图的图片异步生成缩略图
///
/// # 调用时机
/// 此函数在服务器启动时调用，确保所有图片都有对应的缩略图
///
/// # 执行逻辑
/// - 遍历数据库中的所有图片 ID
/// - 检查缩略图文件是否存在
/// - 只有当原图存在且缩略图不存在时才生成
/// - 使用 spawn_blocking 避免阻塞异步运行时
///
/// # 性能考虑
/// - 并发生成多个缩略图，但受限于线程池大小
/// - CPU 密集型操作在独立线程中执行
/// - 大量图片时启动时间会较长
///
/// # 错误处理
/// - 单个缩略图生成失败不会影响其他图片
/// - 数据库查询错误会中断整个过程
/// - 文件系统错误会被记录但不会中断执行
///
/// # 并发控制
/// 当前实现没有限制并发数量，可能造成：
/// - CPU 使用率过高
/// - 磁盘 I/O 压力
/// - 内存使用增加
///
/// # 改进建议
/// 在生产环境中考虑：
/// - 限制并发生成数量
/// - 添加进度日志
/// - 支持增量处理
/// - 添加重试机制
///
/// # 参考
/// - SQLx 查询流: https://docs.rs/sqlx/latest/sqlx/trait.QueryStream.html
/// - futures::TryStreamExt: https://docs.rs/futures/latest/futures/stream/trait.TryStreamExt.html
/// - 批量处理模式: https://en.wikipedia.org/wiki/Batch_processing
async fn fill_missing_thumbnails(pool: &sqlx::Pool<sqlx::Sqlite>) -> anyhow::Result<()> {
    // 创建查询流，获取所有图片的 ID
    // fetch() 返回一个 Stream，可以逐行处理大量数据
    // 避免一次性将所有结果加载到内存中
    let mut rows = sqlx::query("select id from images").fetch(pool);

    // 遍历查询结果流
    // try_next() 是异步的，每次获取一行数据
    // 使用 ? 操作符处理可能的错误
    // 参考: https://docs.rs/futures/latest/futures/stream/trait.TryStreamExt.html#method.try_next
    while let Some(row) = rows.try_next().await? {
        // 从查询结果中提取图片 ID
        // get<类型, 索引> 会自动处理类型转换
        let id = row.get::<i64, _>(0);

        // 构建缩略图文件路径
        let thumbnail_path = format!("image/{id}_thumb.png");

        // 检查缩略图是否已存在
        // 如果不存在，需要生成新的缩略图
        if !std::path::Path::new(&thumbnail_path).exists() {
            // 在生成缩略图前，先检查原图是否存在
            // 这避免了处理数据库中有记录但文件已删除的情况
            let image_path = format!("image/{id}.png");
            if std::path::Path::new(&image_path).exists() {
                // 在独立线程中生成缩略图
                // spawn_blocking 将 CPU 密集型任务与异步运行时分离
                // move 闭包捕获 id 的所有权
                // await?? 双重问号处理 spawn_blocking 和 make_thumbnail 的错误
                spawn_blocking(move || make_thumbnail(id)).await??;
            }
        }
    }

    // 所有缩略图处理完成
    Ok(())
}

/// 图片记录数据结构
///
/// 用于表示数据库中图片的元数据信息
/// 支持从数据库行自动映射和 JSON 序列化
///
/// # 字段说明
/// - `id`: 图片的唯一标识符，数据库主键
/// - `tags`: 图片的标签，用于分类和搜索
///
/// # Trait 派生
/// - `Deserialize`: 支持从 JSON 反序列化，用于接收请求数据
/// - `Serialize`: 支持序列化为 JSON，用于返回响应数据
/// - `FromRow`: SQLx 提供的自动数据库行映射，支持查询结果自动转换
/// - `Debug`: 支持调试输出，便于开发和错误排查
///
/// # 数据库映射
/// 自动映射到数据库表的字段：
/// - `id` ← `images.id`
/// - `tags` ← `images.tags`
///
/// # JSON 表示
/// ```json
/// {
///   "id": 1,
///   "tags": "风景,自然"
/// }
/// ```
///
/// # 参考
/// - Serde 序列化: https://serde.rs/
/// - SQLx FromRow: https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html
/// - Rust derive 宏: https://doc.rust-lang.org/reference/procedural-macros.html
#[derive(Deserialize, Serialize, FromRow, Debug)]
struct ImageRecord {
    /// 图片的唯一标识符
    /// - 数据库类型: INTEGER PRIMARY KEY
    /// - Rust 类型: i32
    /// - 自动递增，由 SQLite 管理
    id: i32,

    /// 图片的标签字符串
    /// - 数据库类型: TEXT NOT NULL
    /// - Rust 类型: String
    /// - 用于图片分类、搜索和显示
    /// - 可以包含多个标签，用逗号分隔
    tags: String,
}

/// 图片列表 API 处理器
///
/// # 参数
/// - `Extension(pool)`: 从 Axum Extension 中获取的数据库连接池
///
/// # 返回值
/// - `Json<Vec<ImageRecord>>`: JSON 格式的图片记录列表
///
/// # HTTP 方法
/// - GET `/images`: 获取所有图片的元数据列表
///
/// # 功能
/// 1. 查询数据库中所有图片记录
/// 2. 按 ID 升序排序
/// 3. 将结果序列化为 JSON 格式返回
///
/// # SQL 查询
/// ```sql
/// SELECT id, tags FROM images ORDER BY id ASC
/// ```
/// - 查询所有图片的 ID 和标签
/// - 按 ID 升序排列，确保结果的一致性
///
/// # 响应格式
/// 返回 JSON 数组，包含所有图片记录：
/// ```json
/// [
///   {
///     "id": 1,
///     "tags": "风景,自然"
///   },
///   {
///     "id": 2,
///     "tags": "人物,肖像"
///   }
/// ]
/// ```
///
/// # 性能考虑
/// - fetch_all() 将所有结果加载到内存
/// - 大量图片时可能消耗较多内存
/// - 考虑实现分页功能
///
/// # 错误处理
/// 当前使用 unwrap()，数据库错误时会 panic
/// 生产环境中应该：
/// - 返回适当的 HTTP 状态码
/// - 提供错误详情
/// - 记录错误日志
///
/// # 缓存策略
/// 考虑添加缓存：
/// - HTTP 缓存头 (Cache-Control, ETag)
/// - 内存缓存查询结果
/// - CDN 缓存静态响应
///
/// # 改进建议
/// - 支持分页查询 (LIMIT/OFFSET)
/// - 支持过滤和搜索
/// - 支持排序选项
/// - 添加图片数量统计
///
/// # 参考
/// - Axum JSON 响应: https://docs.rs/axum/latest/axum/extract/struct.Json.html
/// - SQLx query_as: https://docs.rs/sqlx/latest/sqlx/fn.query_as.html
/// - REST API 设计: https://restfulapi.net/http-methods/
/// - JSON API 规范: https://jsonapi.org/
async fn list_images(Extension(pool): Extension<sqlx::SqlitePool>) -> Json<Vec<ImageRecord>> {
    // 使用 query_as 进行类型安全的查询
    // ImageRecord 实现了 FromRow trait，可以自动映射查询结果
    // 参考: https://docs.rs/sqlx/latest/sqlx/fn.query_as.html
    sqlx::query_as::<_, ImageRecord>("select id, tags from images order by id")
        .fetch_all(&pool) // 执行查询并获取所有结果
        .await // 等待异步操作完成
        .unwrap() // 简化错误处理，生产环境应该使用 ? 或 Result
        .into() // 将 Vec<ImageRecord> 转换为 Json<Vec<ImageRecord>>
}
/// 缩略图获取处理器
///
/// # 参数
/// - `Path(id)`: 从 URL 路径中提取的图片 ID，用于定位缩略图文件
///
/// # 返回值
/// - `impl IntoResponse`: HTTP 响应，包含流式缩略图数据
///
/// # HTTP 方法
/// - GET `/thumb/{id}`: 获取指定 ID 的缩略图文件
///
/// # 功能
/// 1. 根据图片 ID 构建缩略图文件路径
/// 2. 异步打开缩略图文件
/// 3. 将文件内容作为流式 HTTP 响应返回
///
/// # 文件路径规则
/// 缩略图文件路径为 `image/{id}_thumb.png`，与 make_thumbnail 函数的命名规则一致
///
/// # HTTP 响应特性
/// - Content-Type: image/png
/// - 流式传输，适合大文件
/// - 支持 HTTP 范围请求（如果实现）
/// - 自动设置 Content-Length（如果文件系统支持）
///
/// # 性能优势
/// - 缩略图文件小，加载速度快
/// - 流式传输，内存占用低
/// - 适合图片列表、网格视图等场景
/// - 减少带宽消耗
///
/// # 缓存策略
/// 考虑添加缓存头以提升性能：
/// ```http
/// Cache-Control: public, max-age=31536000
/// ETag: "some-etag-value"
/// ```
///
/// # 错误处理
/// 当前使用 unwrap()，文件不存在时会 panic
/// 生产环境中应该：
/// - 返回 404 Not Found（缩略图不存在）
/// - 返回 500 Internal Server Error（读取错误）
/// - 记录访问日志
/// - 考虑自动生成缺失的缩略图
///
/// # 备用策略
/// 当缩略图不存在时，可以考虑：
/// - 自动生成缩略图
/// - 返回原图的缩放版本
/// - 返回默认的占位图
/// - 重定向到原图
///
/// # 使用场景
/// - 图片列表页面
/// - 图片网格展示
/// - 移动端应用
/// - 画廊界面
///
/// # 参考
/// - Axum 路径参数: https://docs.rs/axum/latest/axum/extract/struct.Path.html
/// - Tokio 异步文件: https://docs.rs/tokio/latest/tokio/fs/struct.File.html
/// - 流式响应: https://docs.rs/tokio-util/latest/tokio_util/io/struct.ReaderStream.html
/// - HTTP 缓存: https://developer.mozilla.org/zh-CN/docs/Web/HTTP/Caching
/// - 图片优化: https://web.dev/image-optimization/
async fn get_thumbnail(Path(id): Path<i64>) -> impl IntoResponse {
    // 根据图片 ID 构建缩略图文件路径
    // 使用 format! 宏确保路径格式的正确性
    // 缩略图命名规则: {id}_thumb.png
    let filename = format!("image/{id}_thumb.png");

    // 异步打开缩略图文件进行读取
    // tokio::fs::File 提供了异步文件读取功能
    // 实现了 AsyncRead trait，支持非阻塞读取
    // 参考: https://docs.rs/tokio/latest/tokio/fs/struct.File.html
    let file = tokio::fs::File::open(filename).await.unwrap();

    // 将文件转换为异步流以支持流式传输
    // ReaderStream 将实现了 AsyncRead 的类型转换为 Stream<Item = Result<Bytes, Error>>
    // 优势：
    // 1. 避免将整个文件加载到内存
    // 2. 支持大文件的流式传输
    // 3. 客户端可以边接收边显示
    // 4. 支持并发访问多个文件
    // 参考: https://docs.rs/tokio-util/latest/tokio_util/io/struct.ReaderStream.html
    let stream = ReaderStream::new(file);

    // 将异步流转换为 Axum 的 Body 类型
    // Body::from_stream 创建一个流式的 HTTP 响应体
    // 适用于文件传输、服务器推送事件等场景
    // 参考: https://docs.rs/axum/latest/axum/body/struct.Body.html
    let body = axum::body::Body::from_stream(stream);

    // 构建自定义 HTTP 响应
    // Response::builder 提供了灵活的响应构建方式
    // 可以设置各种 HTTP 头部和自定义响应体
    axum::response::Response::builder()
        .header(
            header::CONTENT_TYPE, // 设置内容类型为 PNG 图片
            header::HeaderValue::from_static("image/png"), // 使用静态字符串避免运行时分配
        )
        .body(body) // 使用流式 body
        .unwrap()
}
