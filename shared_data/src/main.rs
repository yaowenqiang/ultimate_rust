/*
 * Rust 共享数据序列化和网络传输示例
 *
 * 本项目展示了如何在 Rust 中处理共享数据的序列化、反序列化和网络传输：
 * - 使用 serde 进行 JSON 序列化/反序列化
 * - 自定义二进制协议格式
 * - CRC 校验确保数据完整性
 * - 网络字节序处理
 *
 * 📚 官方文档链接：
 *
 * 🔰 基础概念
 * 1. Rust Book - 序列化:
 *    https://doc.rust-lang.org/book/ch12-03-improving-our-io-project.html
 *
 * 2. Rust by Example - JSON:
 *    https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html
 *
 * ⚙️ 序列化库
 * 3. serde 文档:
 *    https://serde.rs/
 *
 * 4. serde_json 文档:
 *    https://docs.rs/serde_json/latest/serde_json/
 *
 * 5. crc32fast 文档:
 *    https://docs.rs/crc32fast/latest/crc32fast/
 *
 * 🚀 高级概念
 * 6. 网络编程:
 *    https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html
 *
 * 7. 字节序处理:
 *    https://doc.rust-lang.org/std/primitive.u16.html#method.to_be_bytes
 *
 * 8. 错误处理:
 *    https://doc.rust-lang.org/book/ch09-00-error-handling.html
 */

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
pub enum collectorCommandV1 {
    /// 提交系统监控数据
    SubmitData {
        collector_id: u32,         // 数据收集器 ID
        total_memory: u64,         // 总内存大小
        used_memory: u64,          // 已使用内存
        average_cpu_usage: f32,    // 平均 CPU 使用率
    },
}

/// 将命令编码为二进制格式（版本 1）
///
/// 这个函数实现了自定义的二进制协议，格式如下：
/// ┌──────────────┬─────────────┬──────────────┬──────────────┬─────────────┬─────────────┐
/// │  Magic Num   │  Version    │  Timestamp   │ Payload Size │   JSON Data  │   CRC32     │
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
/// - Payload Size: JSON 数据长度 (大端序)
/// - JSON Data: JSON 序列化的命令数据
/// - CRC32: JSON 数据的 CRC32 校验和 (大端序)
pub fn encode_v1(command: collectorCommandV1) -> Vec<u8> {
    // 将命令序列化为 JSON 字符串
    let json = serde_json::to_string(&command)
        .expect("JSON 序列化失败");
    let json_bytes = json.as_bytes();

    // 计算 JSON 数据的 CRC32 校验和
    let crc = crc32fast::hash(json_bytes);
    let payload_size = json_bytes.len() as u32;

    // 获取当前时间戳
    let timestamp = unix_now();

    // 预分配足够大的缓冲区
    let mut result = Vec::with_capacity(140);

    // 按照协议顺序写入各个字段（使用大端序）
    result.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());      // 魔数 (2 字节)
    result.extend_from_slice(&VERSION_NUMBER.to_be_bytes());    // 版本号 (2 字节)
    result.extend_from_slice(&timestamp.to_be_bytes());         // 时间戳 (4 字节)
    result.extend_from_slice(&payload_size.to_be_bytes());      // 载荷大小 (4 字节)
    result.extend_from_slice(json_bytes);                       // JSON 数据
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
/// `(u32, collectorCommandV1)` - (时间戳, 解码后的命令)
///
/// # Panics
/// - 如果魔数不匹配
/// - 如果版本号不匹配
/// - 如果 CRC32 校验失败
/// - 如果 JSON 反序列化失败
pub fn decode_v1(bytes: &[u8]) -> (u32, collectorCommandV1) {
    // 解析协议头部（大端序）
    let magic_number = u16::from_be_bytes([bytes[0], bytes[1]]);
    let version_number = u16::from_be_bytes([bytes[2], bytes[3]]);
    let timestamp = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let payload_size = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);

    // 提取 JSON 数据载荷
    let payload_start = 12;
    let payload_end = payload_start + payload_size as usize;
    let payload = &bytes[payload_start..payload_end];

    // 提取 CRC32 校验和
    let crc_start = payload_end;
    let crc_end = crc_start + 4;
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

    // 反序列化 JSON 数据
    let command = serde_json::from_slice(payload)
        .expect("JSON 反序列化失败");

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
        let command = collectorCommandV1::SubmitData {
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
        let command = collectorCommandV1::SubmitData {
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
fn main() {
    println!("🦀 Rust 共享数据序列化演示");
    println!("============================");

    // 创建示例命令
    let sample_command = collectorCommandV1::SubmitData {
        collector_id: 1001,
        total_memory: 8589934592,    // 8 GB
        used_memory: 4294967296,     // 4 GB
        average_cpu_usage: 0.65,     // 65%
    };

    println!("原始命令: {:?}", sample_command);

    // 编码命令
    println!("\n📦 编码数据...");
    let encoded_data = encode_v1(sample_command.clone());
    println!("编码后的数据长度: {} 字节", encoded_data.len());
    println!("编码后的数据 (hex): {:02X?}", encoded_data);

    // 解码数据
    println!("\n📂 解码数据...");
    let (timestamp, decoded_command) = decode_v1(&encoded_data);
    println!("解码后的时间戳: {}", timestamp);
    println!("解码后的命令: {:?}", decoded_command);

    // 验证数据一致性
    println!("\n✅ 验证结果:");
    println!("数据一致性: {}", if decoded_command == sample_command { "✓ 通过" } else { "✗ 失败" });

    // 演示 JSON 格式
    println!("\n📋 JSON 格式:");
    let json_string = serde_json::to_string(&sample_command).unwrap();
    println!("JSON: {}", json_string);

    println!("\n🎯 演示完成！");
    println!("\n💡 提示:");
    println!("  • 运行 `cargo test` 来执行单元测试");
    println!("  • 这个协议可以用于网络通信和数据持久化");
    println!("  • CRC32 校验确保了数据完整性");
}
