# 数据收集服务器 (Server) - 分布式监控系统服务端

## 项目概述

这是一个用 Rust 实现的高性能数据收集服务器，作为分布式监控系统的核心组件。该项目展示了 Rust 在现代 Web 开发、异步编程和数据库操作方面的强大能力，是学习企业级 Rust 开发的绝佳案例。

## 核心功能

- 🌐 **多协议支持**: 同时提供 TCP 数据接收和 REST API 查询
- ⚡ **高性能异步**: 基于 Tokio 异步运行时，支持高并发处理
- 💾 **数据持久化**: 使用 SQLite 数据库存储时序数据
- 🔒 **类型安全**: 利用 Rust 类型系统防止运行时错误
- 📊 **REST API**: 提供完整的数据查询和分析接口
- 🔄 **实时处理**: 实时接收和处理收集器数据

## 技术架构

### 系统组件设计

```
┌─────────────────────────────────────────────────────────────┐
│                     服务器架构                                │
├─────────────────┬─────────────────┬─────────────────────────┤
│   TCP 服务器     │   HTTP 服务器    │      SQLite 数据库      │
│                │                │                        │
│ • 数据接收      │ • REST API     │ • 时序数据存储          │
│ • 协议解析      │ • JSON 响应     │ • 数据持久化            │
│ • 并发连接      │ • 查询接口      │ • 数据完整性            │
│ • 错误处理      │ • 状态管理      │ • 索引优化              │
└─────────────────┴─────────────────┴─────────────────────────┘
         │                   │                   │
         ▼                   ▼                   ▼
┌─────────────────────────────────────────────────────────────┐
│                   异步任务管理                                │
│                • Tokio 运行时                                │
│                • 任务调度                                     │
│                • 资源管理                                     │
└─────────────────────────────────────────────────────────────┘
```

### 数据流程架构

```
收集器客户端 → TCP 连接 → 协议解码 → 数据验证 → 数据库存储
     ↓
HTTP 客户端 → REST API → SQL 查询 → 数据聚合 → JSON 响应
```

## 学习目标

通过这个项目，你将掌握：

### Rust 高级特性
- **异步编程**: async/await、Future、异步运行时
- **并发处理**: 多任务并发、资源共享、同步原语
- **错误处理**: anyhow、Result 类型、错误传播
- **生命周期**: 复杂的借用检查和生命周期管理

### Web 开发技能
- **Web 框架**: Axum 路由、中间件、请求处理
- **REST API**: 资源设计、状态码、内容协商
- **JSON 处理**: Serde 序列化、数据结构设计
- **中间件系统**: 扩展、依赖注入、请求处理链

### 数据库操作
- **异步数据库**: SQLx 异步查询、连接池管理
- **数据建模**: 表结构设计、关系映射、索引优化
- **查询优化**: 参数化查询、性能调优、事务处理
- **迁移管理**: 版本控制、模式演化

### 网络编程
- **TCP 编程**: 异步 TCP、连接管理、协议处理
- **HTTP 服务**: 请求路由、响应处理、状态管理
- **协议设计**: 二进制协议、数据验证、错误处理
- **并发连接**: 连接池、负载均衡、资源限制

## 依赖库详解

### Tokio
- **用途**: 异步运行时和生态系统
- **功能**: 异步 I/O、任务调度、网络编程
- **特性**: `features = ["full"]` 包含完整功能集
- **文档**: https://tokio.rs/tokio/tutorial

### Axum
- **用途**: 现代 Web 框架
- **功能**: 路由、中间件、请求/响应处理
- **特点**: 类型安全、高性能、基于 Tokio
- **文档**: https://docs.rs/axum/latest/axum/

### SQLx
- **用途**: 异步 SQL 工具包
- **功能**: 编译时 SQL 验证、异步查询、连接池
- **特性**: `runtime-tokio-native-tls`, `sqlite`
- **文档**: https://docs.rs/sqlx/latest/sqlx/

### 其他关键依赖
- **anyhow**: 简化错误处理
- **serde**: JSON 序列化和反序列化
- **uuid**: UUID 生成和解析
- **dotenv**: 环境变量管理
- **futures**: 异步编程工具

## 运行指南

### 环境准备

1. **安装 Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **安装 SQLite**:
   ```bash
   # macOS
   brew install sqlite

   # Ubuntu
   sudo apt-get install sqlite3
   ```

### 基本运行

```bash
# 在 server 目录下
# 1. 安装依赖
cargo build

# 2. 运行开发版本
cargo run

# 3. 运行发布版本（推荐）
cargo run --release
```

### 配置文件

创建 `.env` 文件：
```bash
# .env 文件内容
DATABASE_URL="sqlite:collection.db"
```

### API 测试

服务器启动后，可以通过以下 API 端点访问数据：

```bash
# 获取所有数据点
curl http://127.0.0.1:3080/api/all

# 获取收集器列表
curl http://127.0.0.1:3080/api/collectors

# 获取特定收集器的数据
curl http://127.0.0.1:3080/api/collector/550e8400-e29b-41d4-a716-446655440000
```

### 开发调试

```bash
# 代码检查
cargo check

# 格式化代码
cargo fmt

# 静态分析
cargo clippy

# 运行测试
cargo test

# 数据库迁移
sqlx migrate run
```

## API 文档

### 端点概览

| 方法 | 端点 | 描述 | 响应格式 |
|------|------|------|----------|
| GET | `/api/all` | 获取所有数据点 | `DataPoint[]` |
| GET | `/api/collectors` | 获取收集器列表 | `Collector[]` |
| GET | `/api/collector/{uuid}` | 获取特定收集器数据 | `DataPoint[]` |

### 数据模型

#### DataPoint
```json
{
  "id": 1,
  "collector_id": "550e8400-e29b-41d4-a716-446655440000",
  "received": 1697123456,
  "total_memory": 8589934592,
  "used_memory": 4294967296,
  "average_cpu": 0.45
}
```

#### Collector
```json
{
  "id": 1,
  "collector_id": "550e8400-e29b-41d4-a716-446655440000",
  "last_seen": 1697123456
}
```

### HTTP 状态码

- `200 OK`: 请求成功
- `404 Not Found`: 资源不存在
- `500 Internal Server Error`: 服务器内部错误

## 数据库设计

### 表结构

```sql
CREATE TABLE timeseries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    collector_id VARCHAR(255) NOT NULL,
    received INTEGER NOT NULL,
    total_memory UNSIGNED BIG INT NOT NULL,
    used_memory UNSIGNED BIG INT NOT NULL,
    average_cpu REAL NOT NULL
);
```

### 推荐索引

```sql
-- 收集器查询优化
CREATE INDEX idx_collector_id ON timeseries(collector_id);

-- 时间范围查询优化
CREATE INDEX idx_received ON timeseries(received);

-- 复合查询优化
CREATE INDEX idx_collector_time ON timeseries(collector_id, received);
```

## 文件结构

```
server/
├── src/
│   ├── main.rs          # 主程序和 API 路由
│   └── collector.rs     # TCP 数据收集模块
├── migrations/          # 数据库迁移文件
│   └── 20251012074537_initial.sql
├── Cargo.toml           # 项目配置和依赖
├── .env                 # 环境变量配置
├── collection.db        # SQLite 数据库文件（运行时生成）
└── README.md           # 项目文档
```

## 性能优化

### 数据库优化

1. **索引策略**:
   ```sql
   -- 针对常用查询模式创建索引
   CREATE INDEX idx_timeseries_collector_received
   ON timeseries(collector_id, received DESC);
   ```

2. **查询优化**:
   ```rust
   // 使用预编译语句
   let query = sqlx::query_as!(DataPoint,
       "SELECT * FROM timeseries WHERE collector_id = ? ORDER BY received DESC LIMIT ?"
   );
   ```

3. **连接池配置**:
   ```rust
   let pool = SqlitePoolOptions::new()
       .max_connections(20)
       .connect(&database_url)
       .await?;
   ```

### 应用优化

1. **异步并发**:
   - 使用 Tokio 的并发原语
   - 避免阻塞操作
   - 合理设置任务数量

2. **内存管理**:
   - 使用流式处理大数据集
   - 避免不必要的克隆
   - 及时释放资源

3. **网络优化**:
   - 连接复用
   - 请求缓冲
   - 超时设置

## 扩展练习

### 初级练习
1. **日志系统**: 集成 `tracing` 和 `tracing-subscriber`
2. **配置管理**: 使用 `config` crate 支持 TOML/YAML 配置
3. **健康检查**: 添加 `/health` 端点
4. **错误页面**: 改进错误响应格式

### 中级练习
1. **分页查询**: 支持大数据集的分页加载
2. **时间过滤**: 添加时间范围查询参数
3. **数据统计**: 实现统计 API（平均值、最大值等）
4. **缓存系统**: 集成 Redis 缓存热点数据

### 高级练习
1. **WebSocket**: 实现实时数据推送
2. **认证授权**: 添加 JWT 认证系统
3. **限流保护**: 实现请求频率限制
4. **监控指标**: 集成 Prometheus 指标
5. **集群部署**: 支持多实例部署

## 故障排除

### 常见问题

**Q: 数据库连接失败**
```bash
Error: DatabaseConnectionError
```
A: 检查 `DATABASE_URL` 环境变量和 SQLite 文件权限

**Q: TCP 端口被占用**
```bash
Error: Address already in use (os error 48)
```
A: 修改 `DATA_COLLECTOR_ADDRESS` 或停止占用端口的进程

**Q: 编译错误**
```bash
Error: sqlx migrate info failed
```
A: 运行 `sqlx migrate run` 初始化数据库

**Q: 内存使用过高**
A:
- 检查连接池大小设置
- 使用流式查询代替 `fetch_all()`
- 添加查询限制

### 调试技巧

1. **启用详细日志**:
   ```rust
   env_logger::init();
   ```

2. **数据库查询调试**:
   ```rust
   println!("SQL: {}", sqlx::query("...").sql());
   ```

3. **性能分析**:
   ```bash
   # 使用 tokio-console
   cargo install tokio-console
   tokio-console
   ```

4. **内存分析**:
   ```bash
   # 使用 valgrind（需要 Linux 环境）
   valgrind --tool=massif ./target/release/server
   ```

## 安全考虑

### 网络安全
- 使用 HTTPS/TLS 加密通信
- 实现请求速率限制
- 添加输入验证和清理

### 数据安全
- 定期备份数据库
- 实现数据访问控制
- 加密敏感数据

### 应用安全
- 定期更新依赖包
- 使用安全的随机数生成
- 实现错误处理不泄露信息

## 相关资源

### Rust 官方资源
- [Rust 程序设计语言](https://doc.rust-lang.org/book/)
- [Rust 异步编程](https://rust-lang.github.io/async-book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### Web 开发资源
- [Axum 文档](https://docs.rs/axum/latest/axum/)
- [Tokio 教程](https://tokio.rs/tokio/tutorial)
- [HTTP 协议规范](https://tools.ietf.org/html/rfc7231)

### 数据库资源
- [SQLx 文档](https://docs.rs/sqlx/latest/sqlx/)
- [SQLite 文档](https://www.sqlite.org/docs.html)
- [SQL 迁移最佳实践](https://github.com/golang-migrate/migrate)

### 监控和运维
- [Prometheus 监控](https://prometheus.io/)
- [时序数据库概念](https://en.wikipedia.org/wiki/Time_series_database)
- [分布式系统设计](https://en.wikipedia.org/wiki/Distributed_system)

## 贡献指南

欢迎提交 Issue 和 Pull Request！

1. Fork 项目仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启 Pull Request

### 代码规范
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 进行静态分析
- 添加适当的注释和文档
- 编写单元测试和集成测试

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 更新日志

### v0.1.0 (当前版本)
- ✨ 基础 TCP 数据接收功能
- 🌐 REST API 数据查询接口
- 💾 SQLite 数据库持久化
- ⚡ 异步并发处理
- 🔧 基础配置和环境管理

### 计划功能 (v0.2.0)
- 📊 数据统计和分析 API
- 🔄 WebSocket 实时推送
- 🔐 JWT 认证系统
- 📈 Prometheus 监控指标
- 🛡️ 请求限流和安全增强

---

**注意**: 这是一个学习项目，生产环境使用请确保进行充分的安全审计和性能测试。