-- 数据库初始化迁移脚本
--
-- 这个脚本创建了监控系统的主要数据表，用于存储来自各个收集器的性能数据。
--
-- 迁移版本：20251012074537_initial
-- 创建时间：2025-10-12 07:45:37
-- 数据库：SQLite
--
-- 相关文档：
-- - SQLx 迁移：https://docs.rs/sqlx/latest/sqlx/migrate/
-- - SQLite 数据类型：https://www.sqlite.org/datatype3.html
-- - 时序数据库概念：https://en.wikipedia.org/wiki/Time_series_database

-- 创建时间序列数据表
-- 这个表存储所有收集器上报的性能监控数据
create table if not exists timeseries (
    -- 主键：自增整数，每条记录的唯一标识符
    -- SQLite 中的 INTEGER PRIMARY KEY 自动具有 AUTOINCREMENT 属性
    id serial primary key,

    -- 收集器标识符：UUID 字符串格式
    -- 用于标识数据来源的收集器实例
    -- varchar(255) 足够存储完整的 UUID (36 字符)
    collector_id varchar(255),

    -- 数据接收时间戳：Unix 时间戳格式
    -- 表示服务器接收到这条数据的时间
    -- 使用 INTEGER 存储 Unix 时间戳（秒），便于时间范围查询
    received timestamp,

    -- 系统总内存：无符号大整数
    -- 表示收集器上报的系统总物理内存大小（字节）
    -- 使用 UNSIGNED BIG INT 支持大容量内存系统（最大 16EB）
    total_memory unsigned big int,

    -- 已使用内存：无符号大整数
    -- 表示收集器上报的当前已使用内存大小（字节）
    -- 与 total_memory 一起可以计算内存使用率
    used_memory unsigned big int,

    -- 平均 CPU 使用率：浮点数
    -- 表示收集器在采样周期内的平均 CPU 使用率
    -- 范围通常是 0.0 到 1.0（或 0% 到 100%）
    average_cpu float
);

-- 推荐的索引（可以后续添加）：
-- -- 收集器 ID 索引，加速按收集器查询
-- CREATE INDEX idx_timeseries_collector_id ON timeseries(collector_id);
--
-- -- 时间戳索引，加速时间范围查询
-- CREATE INDEX idx_timeseries_received ON timeseries(received);
--
-- -- 复合索引，加速特定收集器的时间范围查询
-- CREATE INDEX idx_timeseries_collector_time ON timeseries(collector_id, received);