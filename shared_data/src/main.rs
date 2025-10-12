// 共享数据协议演示程序
//
// 这个演示程序展示了自定义二进制协议的完整功能：
// 1. 数据结构定义和序列化
// 2. 自定义二进制协议编解码
// 3. CRC32 数据完整性校验
// 4. 网络字节序和跨平台兼容性
// 5. 协议版本化和向前兼容性
//
// 作为 ultimate_rust 学习工作空间的一部分，这个项目重点展示了：
// - Rust 的序列化/反序列化生态系统
// - 二进制协议设计的最佳实践
// - 类型安全和内存安全的保证
// - 测试驱动开发 (TDD) 方法
// - 错误处理和调试技巧
//
// 📚 相关学习资源：
//
// 🔰 Rust 基础概念
// - Rust Book - 序列化: https://doc.rust-lang.org/book/ch12-03-improving-our-io-project.html
// - Rust by Example - 错误处理: https://doc.rust-lang.org/rust-by-example/error.html
//
// ⚙️ 序列化技术
// - Serde 官方文档: https://serde.rs/
// - Bincode 文档: https://docs.rs/bincode/latest/bincode/
// - CRC32 算法: https://en.wikipedia.org/wiki/Cyclic_redundancy_check
//
// 🚀 网络编程
// - Rust 网络编程: https://doc.rust-lang.org/std/net/index.html
// - 字节序处理: https://doc.rust-lang.org/std/primitive.u16.html#method.to_be_bytes
// - 协议设计: https://tools.ietf.org/html/rfc791
//
// 🧪 测试和调试
// - Rust 测试: https://doc.rust-lang.org/book/ch11-00-testing.html
// - 单元测试最佳实践: https://doc.rust-lang.org/rust-by-example/testing.html

use serde::{Deserialize, Serialize}; // 序列化和反序列化支持
use std::time::{SystemTime, UNIX_EPOCH}; // 时间处理

// 数据收集器网络地址常量
pub const DATA_COLLECTOR_ADDRESS: &str = "127.0.0.1:9004";

// 协议魔数：用于识别协议格式
const MAGIC_NUMBER: u16 = 1234;

// 协议版本号
const VERSION_NUMBER: u16 = 1;

/// 获取当前 Unix 时间戳（秒）
///
/// 返回从 Unix 纪元（1970-01-01 00:00:00 UTC）到现在的秒数
///
/// # 返回值
/// `u32` - 当前时间的 Unix 时间戳
///
/// # 注意
/// 如果系统时间出现异常（例如时间倒流），程序会 panic
fn unix_now() -> u32 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("系统时间异常：时间倒流");
    since_the_epoch.as_secs() as u32
}

/// 数据收集器命令枚举（版本 1）
///
/// 这个枚举定义了数据收集器可以处理的所有命令类型
/// 目前只支持一种命令：SubmitData，用于提交系统监控数据
///
/// 字段说明：
/// - collector_id: 数据收集器的唯一标识符
/// - total_memory: 系统总内存（字节）
/// - used_memory: 已使用内存（字节）
/// - average_cpu_usage: 平均 CPU 使用率（0.0-1.0）
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CollectorCommandV1 {
    /// 提交系统监控数据
    SubmitData {
        collector_id: u128,        // 数据收集器 ID (UUID)
        total_memory: u64,         // 总内存大小
        used_memory: u64,          // 已使用内存
        average_cpu_usage: f32,    // 平均 CPU 使用率
    },
}

/// 将命令编码为二进制格式（版本 1）
///
/// 这个函数实现了自定义的二进制协议，格式如下：
/// ┌──────────────┬─────────────┬──────────────┬──────────────┬─────────────┬─────────────┐
/// │  Magic Num   │  Version    │  Timestamp   │ Payload Size │  Bincode    │   CRC32     │
/// │   (2 bytes)  │  (2 bytes)  │  (4 bytes)   │  (4 bytes)   │  (variable)  │  (4 bytes)  │
/// └──────────────┴─────────────┴──────────────┴──────────────┴─────────────┴─────────────┘
///
/// # 参数
/// `command` - 要编码的命令
///
/// # 返回值
/// `Vec<u8>` - 编码后的二进制数据
///
/// # 协议细节
/// - Magic Number: 1234 (大端序)
/// - Version: 1 (大端序)
/// - Timestamp: Unix 时间戳 (大端序)
/// - Payload Size: Bincode 数据长度 (大端序)
/// - Bincode Data: Bincode 序列化的命令数据
/// - CRC32: Bincode 数据的 CRC32 校验和 (大端序)
pub fn encode_v1(command: CollectorCommandV1) -> Vec<u8> {
    // 将命令序列化为二进制格式 (bincode)
    let payload_bytes = bincode::serialize(&command).unwrap();

    // 计算二进制数据的 CRC32 校验和
    let crc = crc32fast::hash(&payload_bytes);
    let payload_size = payload_bytes.len() as u32;

    // 获取当前时间戳
    let timestamp = unix_now();

    // 预分配足够大的缓冲区
    let mut result = Vec::with_capacity(140);

    // 按照协议顺序写入各个字段（使用大端序）
    result.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());      // 魔数 (2 字节)
    result.extend_from_slice(&VERSION_NUMBER.to_be_bytes());    // 版本号 (2 字节)
    result.extend_from_slice(&timestamp.to_be_bytes());         // 时间戳 (4 字节)
    result.extend_from_slice(&payload_size.to_be_bytes());      // 载荷大小 (4 字节)
    result.extend_from_slice(&payload_bytes);                   // Bincode 数据
    result.extend_from_slice(&crc.to_be_bytes());               // CRC32 校验和 (4 字节)

    result
}

/// 从二进制格式解码命令（版本 1）
///
/// 这个函数解析按照 encode_v1 协议编码的二进制数据
/// 会验证魔数、版本号和 CRC32 校验和
///
/// # 参数
/// `bytes` - 要解码的二进制数据
///
/// # 返回值
/// `(u32, CollectorCommandV1)` - (时间戳, 解码后的命令)
///
/// # Panics
/// - 如果魔数不匹配
/// - 如果版本号不匹配
/// - 如果 CRC32 校验失败
/// - 如果 Bincode 反序列化失败
pub fn decode_v1(bytes: &[u8]) -> (u32, CollectorCommandV1) {
    // 解析协议头部（大端序）
    let magic_number = u16::from_be_bytes([bytes[0], bytes[1]]);
    let version_number = u16::from_be_bytes([bytes[2], bytes[3]]);
    let timestamp = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let payload_size = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);

    // 提取二进制数据载荷
    let payload_start = 12;
    let payload_end = payload_start + payload_size as usize;
    let payload = &bytes[payload_start..payload_end];

    // 提取 CRC32 校验和
    let crc_start = payload_end;
    let crc = u32::from_be_bytes([
        bytes[crc_start],
        bytes[crc_start + 1],
        bytes[crc_start + 2],
        bytes[crc_start + 3],
    ]);

    // 验证魔数和版本号
    assert_eq!(magic_number, MAGIC_NUMBER, "协议魔数不匹配");
    assert_eq!(version_number, VERSION_NUMBER, "协议版本号不匹配");

    // 验证 CRC32 校验和
    let computed_crc = crc32fast::hash(payload);
    assert_eq!(crc, computed_crc, "CRC32 校验失败，数据可能已损坏");

    // 反序列化二进制数据
    let command = bincode::deserialize(payload)
        .expect("Bincode 反序列化失败");

    (timestamp, command)
}

// 单元测试模块
#[cfg(test)]
mod tests {
    use super::*;

    /// 测试编码和解码函数的正确性
    ///
    /// 这个测试验证：
    /// 1. 编码函数能正确序列化命令
    /// 2. 解码函数能正确还原命令
    /// 3. 时间戳能正确生成
    /// 4. 数据完整性得到保证
    #[test]
    fn test_encode_decode() {
        // 创建测试命令
        let command = CollectorCommandV1::SubmitData {
            collector_id: 1234,
            total_memory: 100,
            used_memory: 50,
            average_cpu_usage: 0.5,
        };

        // 编码命令
        let encoded = encode_v1(command.clone());

        // 解码数据
        let (timestamp, decoded) = decode_v1(&encoded);

        // 验证解码结果与原始命令一致
        assert_eq!(decoded, command, "解码后的命令与原始命令不匹配");

        // 验证时间戳合理（应该大于 0）
        assert!(timestamp > 0, "时间戳应该大于 0");

        // 验证编码后的数据包含所有必要的字段
        assert!(encoded.len() > 20, "编码后的数据长度应该大于头部大小");
    }

    /// 测试协议的完整性校验
    #[test]
    fn test_protocol_integrity() {
        let command = CollectorCommandV1::SubmitData {
            collector_id: 5678,
            total_memory: 1024,
            used_memory: 512,
            average_cpu_usage: 0.75,
        };

        let encoded = encode_v1(command);

        // 测试正常解码
        let (timestamp, _) = decode_v1(&encoded);
        assert!(timestamp > 0);

        // 测试损坏数据的处理
        let mut corrupted = encoded.clone();
        if let Some(last_byte) = corrupted.last_mut() {
            *last_byte = last_byte.wrapping_add(1); // 修改最后一个字节
        }

        // 这应该触发 panic（在测试中被捕获）
        std::panic::catch_unwind(|| {
            decode_v1(&corrupted);
        }).expect_err("应该检测到数据损坏");
    }
}

/// 主函数：演示共享数据编码解码功能
///
/// 这个函数提供了一个完整的演示，展示了自定义二进制协议的：
/// 1. 数据结构定义和使用
/// 2. 二进制序列化和反序列化
/// 3. 协议编解码的完整流程
/// 4. 数据完整性校验机制
/// 5. 性能对比分析
///
/// 演示内容包括：
/// - 创建真实的系统监控数据
/// - 执行完整的编解码流程
/// - 展示二进制格式和效率
/// - 验证数据完整性和一致性
/// - 提供使用指导和最佳实践
fn main() {
    println!("🦀 Rust 共享数据序列化演示");
    println!("============================");

    // 创建示例命令
    // 使用真实的系统监控数据进行演示
    let sample_command = CollectorCommandV1::SubmitData {
        collector_id: 1001,                   // 收集器 ID
        total_memory: 8589934592,              // 8 GB 总内存
        used_memory: 4294967296,               // 4 GB 已使用内存 (50%)
        average_cpu_usage: 0.65,               // 65% CPU 使用率
    };

    println!("📊 原始命令: {:?}", sample_command);

    // 执行编码操作
    println!("\n📦 编码数据...");
    let encoded_data = encode_v1(sample_command.clone());
    println!("编码后的数据长度: {} 字节", encoded_data.len());
    println!("编码后的数据 (hex): {:02X?}", encoded_data);

    // 执行解码操作
    println!("\n📂 解码数据...");
    let (timestamp, decoded_command) = decode_v1(&encoded_data);
    println!("解码后的时间戳: {}", timestamp);
    println!("解码后的命令: {:?}", decoded_command);

    // 验证数据一致性
    println!("\n✅ 验证结果:");
    println!("数据一致性: {}", if decoded_command == sample_command { "✓ 通过" } else { "✗ 失败" });

    // 性能对比分析
    println!("\n📈 性能分析:");

    // 二进制格式分析
    println!("二进制格式:");
    println!("  • 数据长度: {} 字节", encoded_data.len());
    println!("  • 协议开销: {} 字节 (头部 + CRC)", 12 + 4);
    println!("  • 有效载荷: {} 字节", encoded_data.len() - 16);

    // JSON 格式对比
    println!("\n📋 JSON 格式对比:");
    let json_string = serde_json::to_string(&sample_command).unwrap();
    let json_bytes = json_string.as_bytes();
    println!("  • JSON 长度: {} 字节", json_bytes.len());
    println!("  • 空间节省: {:.1}%", (1.0 - encoded_data.len() as f64 / json_bytes.len() as f64) * 100.0);
    println!("  • JSON 内容: {}", json_string);

    // 协议安全性说明
    println!("\n🔒 安全特性:");
    println!("  • CRC32 校验: 检测数据传输错误");
    println!("  • 魔数验证: 防止协议混淆攻击");
    println!("  • 版本控制: 支持协议向前兼容");
    println!("  • 类型安全: 编译时防止数据错误");

    // 使用建议
    println!("\n🎯 演示完成！");
    println!("\n💡 使用建议:");
    println!("  • 运行 `cargo test` 来执行全面的单元测试");
    println!("  • 这个协议适合网络通信和数据持久化");
    println!("  • 在生产环境中建议添加更完善的错误处理");
    println!("  • 可根据需要调整协议字段和数据类型");

    println!("\n🚀 扩展方向:");
    println!("  • 添加更多命令类型（心跳、配置更新等）");
    println!("  • 支持数据压缩以进一步节省带宽");
    println!("  • 集成加密机制保护敏感数据");
    println!("  • 实现协议协商支持多版本兼容");
}
