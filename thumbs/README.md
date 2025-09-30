# Thumbs - 简单的图片上传服务器

一个基于 Rust 和 Axum 框架的高性能图片上传服务器，支持标签管理和 SQLite 数据库存储。

## 🚀 功能特性

- **高性能**: 基于 Tokio 异步运行时和 Axum Web 框架
- **数据库支持**: 使用 SQLite 存储图片元数据
- **文件上传**: 支持多部分表单上传
- **标签管理**: 为图片添加自定义标签
- **流式响应**: 支持大文件的流式传输
- **类型安全**: 使用 Rust 的类型系统确保内存安全

## 🛠 技术栈

| 技术栈 | 用途 | 文档链接 |
|--------|------|----------|
| [Axum](https://docs.rs/axum) | Web 框架 | [GitHub](https://github.com/tokio-rs/axum) |
| [SQLx](https://docs.rs/sqlx) | 数据库操作 | [GitHub](https://github.com/launchbadge/sqlx) |
| [Tokio](https://tokio.rs/tokio/tutorial) | 异步运行时 | [GitHub](https://github.com/tokio-rs/tokio) |
| [Image](https://docs.rs/image) | 图片处理 | [GitHub](https://github.com/image-rs/image) |
| [Serde](https://docs.rs/serde) | 序列化/反序列化 | [GitHub](https://github.com/serde-rs/serde) |
| [Anyhow](https://docs.rs/anyhow) | 错误处理 | [GitHub](https://github.com/dtolnay/anyhow) |

## 📦 安装和运行

### 前置要求

- Rust 1.75+ (推荐使用 rustup 安装)
- SQLite 3

### 克隆和构建

```bash
# 克隆仓库
git clone https://github.com/yourusername/thumbs.git
cd thumbs

# 构建项目
cargo build --release
```

### 配置环境

1. 创建 `.env` 文件：

```env
DATABASE_URL="sqlite:images.db"
```

2. 确保 `migrations/` 目录包含数据库迁移文件

### 运行服务器

```bash
# 开发模式
cargo run

# 生产模式
cargo run --release
```

服务器将在 `http://127.0.0.1:8280` 启动

## 📡 API 端点

### GET `/`
显示图片上传页面

**响应**: HTML 页面

### POST `/upload`
上传图片和标签

**请求**: multipart/form-data
- `tags`: 图片标签（字符串）
- `image`: 图片文件（二进制数据）

**响应**: `"OK"` (成功) 或 错误信息

**示例** (使用 curl):
```bash
curl -X POST http://127.0.0.1:8280/upload \
  -F "tags=nature,landscape" \
  -F "image=@/path/to/image.jpg"
```

### GET `/image/{id}`
获取指定 ID 的图片

**路径参数**:
- `id`: 图片的数据库 ID

**响应**: 图片文件 (image/png)

**示例**:
```bash
curl -o image.png http://127.0.0.1:8280/image/1
```

## 🗄 数据库结构

### images 表

| 列名 | 类型 | 说明 |
|------|------|------|
| id | INTEGER | 主键，自动递增 |
| tags | TEXT | 图片标签，逗号分隔 |

### 迁移文件示例

```sql
-- migrations/20250928151328_initial.sql
CREATE TABLE IF NOT EXISTS images (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tags TEXT NOT NULL
);
```

## 📁 项目结构

```
thumbs/
├── src/
│   ├── main.rs          # 主程序文件
│   └── index.html       # 上传页面模板
├── migrations/          # 数据库迁移文件
│   └── 20250928151328_initial.sql
├── image/              # 图片存储目录（运行时创建）
├── .env               # 环境变量配置
├── Cargo.toml         # 项目配置和依赖
└── README.md          # 项目文档
```

## 🔧 开发指南

### 添加新的 API 端点

1. 在 `main.rs` 中定义处理函数：

```rust
async fn new_handler() -> impl IntoResponse {
    // 处理逻辑
}
```

2. 在路由中注册：

```rust
let app = Router::new()
    .route("/new-endpoint", get(new_handler))
    // ... 其他路由
```

### 数据库操作

使用 SQLx 进行类型安全的数据库操作：

```rust
// 插入数据
let result = sqlx::query("INSERT INTO table (column) VALUES (?)")
    .bind(value)
    .execute(&pool)
    .await?;

// 查询数据
let row = sqlx::query("SELECT * FROM table WHERE id = ?")
    .bind(id)
    .fetch_one(&pool)
    .await?;

let value: String = row.get("column");
```

### 错误处理

使用 Anyhow 进行错误处理：

```rust
use anyhow::Result;

async fn some_function() -> Result<String> {
    let result = some_operation()?;
    Ok(result)
}
```

## 🧪 测试

运行测试：

```bash
cargo test
```

## 🚀 部署

### 使用 Docker

创建 `Dockerfile`:

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y sqlite3 ca-certificates
COPY --from=builder /app/target/release/thumbs /usr/local/bin/
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/src/index.html /app/index.html
WORKDIR /app
EXPOSE 8280
CMD ["thumbs"]
```

构建和运行：

```bash
docker build -t thumbs .
docker run -p 8280:8280 -e DATABASE_URL="sqlite:images.db" thumbs
```

### 使用 systemd

创建服务文件 `/etc/systemd/system/thumbs.service`:

```ini
[Unit]
Description=Thumbs Image Upload Server
After=network.target

[Service]
Type=simple
User=thumbs
WorkingDirectory=/opt/thumbs
ExecStart=/opt/thumbs/target/release/thumbs
Restart=always
RestartSec=5
Environment=DATABASE_URL="sqlite:/opt/thumbs/images.db"

[Install]
WantedBy=multi-user.target
```

## 🤝 贡献

欢迎贡献代码！请遵循以下步骤：

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [Axum](https://github.com/tokio-rs/axum) - 优秀的 Web 框架
- [SQLx](https://github.com/launchbadge/sqlx) - 类型安全的 SQL 工具包
- [Tokio](https://github.com/tokio-rs/tokio) - 强大的异步运行时

## 📞 联系

- 项目主页: [https://github.com/yourusername/thumbs](https://github.com/yourusername/thumbs)
- 问题反馈: [Issues](https://github.com/yourusername/thumbs/issues)
- 邮箱: your.email@example.com

---

⭐ 如果这个项目对你有帮助，请给它一个星标！