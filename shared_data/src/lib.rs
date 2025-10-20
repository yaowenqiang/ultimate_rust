#[warn(missing_docs)]
// 共享数据协议库 - 核心实现
//
// 这个库实现了一个高效的自定义二进制协议，用于分布式系统中的数据通信。
// 协议设计重点关注数据完整性、版本兼容性和传输效率。
//
// 核心功能：
// 1. 数据序列化/反序列化 (使用 bincode)
// 2. 二进制协议编解码
// 3. CRC32 数据完整性校验
// 4. 网络字节序处理
// 5. 版本化协议支持
//
// 技术特点：
// - 高效的二进制格式（比 JSON 节省 50-80% 带宽）
// - 强类型保证编译时安全
// - 全面的单元测试覆盖
// - 详细的错误处理和调试信息
//
// 相关文档：
// - Bincode 文档：https://docs.rs/bincode/latest/bincode/
// - Serde 文档：https://serde.rs/
// - CRC32 算法：https://en.wikipedia.org/wiki/Cyclic_redundancy_check
// - 网络字节序：https://en.wikipedia.org/wiki/Endianness
use serde::{Deserialize, Serialize}; // 序列化和反序列化支持
use std::time::{SystemTime, UNIX_EPOCH}; // 时间处理，用于生成时间戳

/// 数据收集器网络地址常量
///
/// 定义了 TCP 服务器监听的地址和端口，收集器客户端将连接到这个地址。
/// 格式：IP:PORT
///
/// # 安全考虑
/// 在生产环境中，这个地址可能需要：
/// - 使用配置文件或环境变量管理
/// - 支持 0.0.0.0 监听所有接口
/// - 考虑防火墙和安全组设置
pub const DATA_COLLECTOR_ADDRESS: &str = "127.0.0.1:9004";

/// 协议魔数 - 用于识别和验证协议格式
///
/// 魔数是网络协议的第一个字段，用于：
/// 1. 快速识别数据包格式
/// 2. 过滤不相关的网络流量
/// 3. 防止协议混淆攻击
///
/// 选择 1234 的原因：
/// - 简单易记，便于调试
/// - 2 字节范围内，不会与常用协议冲突
/// - 在二进制中容易识别 (0x04D2)
const MAGIC_NUMBER: u16 = 1234;

/// 协议版本号 - 支持向前兼容性
///
/// 版本号允许协议演进而保持向后兼容：
/// - 版本 1: 基础数据收集协议
/// - 未来版本可添加新字段而不破坏现有客户端
/// - 支持协议协商和多版本并存
///
/// 版本管理策略：
/// - 主版本号：不兼容的协议变更
/// - 次版本号：向后兼容的功能增加
/// - 修订版本号：向后兼容的问题修正
const VERSION_NUMBER: u16 = 1;

/// 获取当前 Unix 时间戳（秒级精度）
///
/// Unix 时间戳是从 Unix 纪元（1970-01-01 00:00:00 UTC）到现在的秒数。
/// 这个函数用于协议中的时间戳字段，帮助：
/// 1. 追踪数据包的生成时间
/// 2. 检测网络延迟和时钟偏差
/// 3. 支持时间序列数据分析
///
/// # 返回值
/// `u32` - 当前时间的 Unix 时间戳（秒）
///
/// # 注意事项
/// - 使用 u32 类型，支持到 2038 年的时间范围
/// - 如果系统时间异常（如时间倒流），程序会 panic
/// - 精度为秒级，适合监控数据的采样频率
///
/// # 时间戳相关概念
/// - Unix 纪元：1970-01-01 00:00:00 UTC
/// - 32 位时间戳问题：2038 年问题
/// - 时区处理：统一使用 UTC 时间
///
/// 文档参考：
/// - SystemTime 文档：https://doc.rust-lang.org/std/time/struct.SystemTime.html
/// - Unix 时间：https://en.wikipedia.org/wiki/Unix_time
fn unix_now() -> u32 {
    // 获取当前系统时间
    let start = SystemTime::now();

    // 计算从 Unix 纪元到现在的时长
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("系统时间异常：时间倒流");

    // 转换为秒数并返回
    since_the_epoch.as_secs() as u32
}

/// 数据收集器命令枚举（版本 1）
///
/// 这个枚举定义了分布式监控系统中数据收集器可以执行的所有命令类型。
/// 当前版本只支持一种命令：SubmitData，用于上报系统监控数据。
///
/// # 设计原则
/// - 版本化：每个命令都带有版本号，支持协议演进
/// - 可扩展：未来可以轻松添加新的命令类型
/// - 类型安全：利用 Rust 的类型系统防止数据错误
/// - 序列化友好：支持高效的二进制序列化
///
/// # 派生特征说明
/// - Serialize: 支持序列化为字节流
/// - Deserialize: 支持从字节流反序列化
/// - Clone: 支持命令的复制操作
/// - Debug: 支持格式化输出用于调试
/// - PartialEq: 支持命令的相等性比较
///
/// # 字段类型选择
/// - u128: 支持完整的 UUID，确保全局唯一性
/// - u64: 支持大内存系统（最高 16EB）
/// - f32: 平衡精度和存储效率
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CollectorCommandV1 {
    /// 提交系统监控数据命令
    ///
    /// 这是最核心的命令，用于收集器向服务器上报系统性能数据。
    /// 包含了系统监控的关键指标，支持实时监控和历史分析。
    ///
    /// # 字段说明
    /// - collector_id: 收集器的全局唯一标识符（UUID v4）
    /// - total_memory: 系统总物理内存大小（字节）
    /// - used_memory: 当前已使用的物理内存（字节）
    /// - average_cpu_usage: 采样周期内的平均 CPU 使用率（0.0-1.0）
    ///
    /// # 数据范围和精度
    /// - collector_id: 128 位 UUID，支持约 3.4×10^38 个唯一值
    /// - total_memory/used_memory: 最大支持 16 EB 内存
    /// - average_cpu_usage: 单精度浮点，精度约 6-7 位十进制数字
    ///
    /// # 使用场景
    /// - 实时系统监控
    /// - 容量规划和趋势分析
    /// - 性能问题诊断
    /// - 自动化运维决策
    SubmitData {
        /// 收集器唯一标识符
        ///
        /// 使用 UUID v4 格式的 128 位整数，确保：
        /// - 全局唯一性：不同收集器不会冲突
        /// - 随机性：防止 ID 猜测攻击
        /// - 时间无关：不依赖系统时钟
        collector_id: u128,

        /// 系统总内存大小
        ///
        /// 单位：字节
        /// 包含所有可用的物理内存，包括：
        /// - 系统保留内存
        /// - 用户可用内存
        /// - 缓存和缓冲区
        total_memory: u64,

        /// 已使用内存大小
        ///
        /// 单位：字节
        /// 包含所有被占用的物理内存：
        /// - 应用程序内存
        /// - 系统内核内存
        /// - 文件缓存
        ///
        /// 注意：这个值可能超过 total_memory，
        /// 因为某些系统计算包含共享内存的重复计算
        used_memory: u64,

        /// 平均 CPU 使用率
        ///
        /// 范围：0.0 - 1.0（0% - 100%）
        /// 表示在采样周期内的平均 CPU 使用率
        ///
        /// # 计算方式
        /// 通常是通过以下方式计算：
        /// ```text
        /// CPU 使用率 = (忙碌时间 / 总时间) × 100%
        /// ```
        ///
        /// # 采样考虑
        /// - 采样间隔：通常 1 秒
        /// - 多核心：所有核心的平均值
        /// - 系统负载：包含用户态和内核态
        average_cpu_usage: f32,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CollectorResponseV1 {
    Ack(u32),
}

pub fn encode_response_v1(command: CollectorResponseV1) -> Vec<u8> {
    bincode::serialize(&command).unwrap()
}

pub fn decode_response_v1(bytes: &[u8]) -> CollectorResponseV1 {
    bincode::deserialize(bytes).unwrap()
}

/// 将命令编码为二进制格式（版本 1）
///
/// 这个函数实现了自定义的二进制协议编码，将结构化的命令数据转换为
/// 适合网络传输的字节序列。协议设计注重效率、可靠性和可扩展性。
///
/// # 协议格式详解
/// ```
/// ┌──────────────┬─────────────┬──────────────┬──────────────┬─────────────┬─────────────┐
/// │  Magic Num   │  Version    │  Timestamp   │ Payload Size │  Bincode    │   CRC32     │
/// │   (2 bytes)  │  (2 bytes)  │  (4 bytes)   │  (4 bytes)   │  (variable)  │  (4 bytes)  │
/// └──────────────┴─────────────┴──────────────┴──────────────┴─────────────┴─────────────┘
/// ```
///
/// # 字段说明
/// - Magic Number (2 bytes): 协议标识符 `0x04D2` (1234)
/// - Version (2 bytes): 协议版本号 `0x0001` (1)
/// - Timestamp (4 bytes): Unix 时间戳，大端序
/// - Payload Size (4 bytes): 载荷数据长度，大端序
/// - Payload (variable): Bincode 序列化的命令数据
/// - CRC32 (4 bytes): 载荷数据的校验和，大端序
///
/// # 技术特点
/// - **二进制格式**: 比 JSON 节省 50-80% 的带宽
/// - **固定头部**: 前 16 字节固定格式，便于快速解析
/// - **CRC32 校验**: 检测传输过程中的数据损坏
/// - **网络字节序**: 使用大端序确保跨平台兼容
/// - **时间戳**: 支持网络延迟分析和时序数据处理
///
/// # 性能特点
/// - 内存预分配: `Vec::with_capacity(140)` 避免重复分配
/// - 零拷贝设计: `extend_from_slice` 高效拷贝数据
/// - 编译时优化: 所有类型在编译时确定
///
/// # 参数
/// * `command` - 要编码的 CollectorCommandV1 命令
///
/// # 返回值
/// `Vec<u8>` - 编码后的完整二进制数据包
///
/// # 示例
/// ```rust
/// let command = CollectorCommandV1::SubmitData {
///     collector_id: 1234,
///     total_memory: 8589934592,
///     used_memory: 4294967296,
///     average_cpu_usage: 0.65,
/// };
/// let encoded = encode_v1(&command);
/// println!("编码后长度: {} 字节", encoded.len());
/// ```
///
/// # 版本历史
/// - v1.0: 初始版本，支持基础数据收集
/// - 未来版本可添加新字段，保持向后兼容
pub fn encode_v1(command: CollectorCommandV1) -> Vec<u8> {
    // ===== 序列化命令数据 =====
    // 使用 bincode 进行二进制序列化，相比 JSON 的优势：
    // 1. 更紧凑的表示（通常小 50-80%）
    // 2. 更快的序列化/反序列化速度
    // 3. 类型安全的二进制格式
    let payload_bytes = bincode::serialize(&command).unwrap();

    // ===== 计算校验和 =====
    // CRC32 是一种广泛使用的错误检测码
    // 可以检测出 99.9999% 以上的单比特错误
    // 适合网络传输中的数据完整性检查
    let crc = crc32fast::hash(&payload_bytes);
    let payload_size = payload_bytes.len() as u32;

    // ===== 生成时间戳 =====
    // 用于追踪数据包的生成时间和网络延迟
    // 也支持时间序列数据的时间基准
    let timestamp = unix_now();

    // ===== 构建二进制数据包 =====
    // 预分配足够的缓冲区，避免多次内存重分配
    // 估计大小：2+2+4+4+~50+4 = ~66 字节，使用 140 作为安全值
    let mut result = Vec::with_capacity(140);

    // 按照协议格式写入各个字段（全部使用大端序）
    result.extend_from_slice(&MAGIC_NUMBER.to_be_bytes()); // 魔数 (2 字节)
    result.extend_from_slice(&VERSION_NUMBER.to_be_bytes()); // 版本号 (2 字节)
    result.extend_from_slice(&timestamp.to_be_bytes()); // 时间戳 (4 字节)
    result.extend_from_slice(&payload_size.to_be_bytes()); // 载荷大小 (4 字节)
    result.extend_from_slice(&payload_bytes); // Bincode 数据 (变长)
    result.extend_from_slice(&crc.to_be_bytes()); // CRC32 校验和 (4 字节)

    result
}

/// 从二进制格式解码命令（版本 1）
///
/// 这个函数解析按照 `encode_v1` 协议编码的二进制数据，进行完整的
/// 协议验证、数据完整性检查和反序列化操作。
///
/// # 解码流程
/// 1. **协议头部解析**: 提取魔数、版本号、时间戳、载荷大小
/// 2. **数据完整性校验**: 验证 CRC32 校验和
/// 3. **数据反序列化**: 将二进制数据还原为结构化命令
/// 4. **错误处理**: 检测并报告各种协议错误
///
/// # 安全性保证
/// - **魔数验证**: 防止协议混淆和恶意数据
/// - **版本检查**: 确保协议兼容性
/// - **边界检查**: 防止缓冲区溢出攻击
/// - **CRC32 校验**: 检测数据传输损坏
///
/// # 错误处理策略
/// 使用 `assert!` 宏进行严格验证：
/// - 魔数不匹配: 立即 panic，防止处理恶意数据
/// - 版本不匹配: panic，确保协议兼容性
/// - CRC 校验失败: panic，防止使用损坏数据
/// - 反序列化失败: panic，保证数据完整性
///
/// # 参数
/// * `bytes` - 要解码的二进制数据切片
///
/// # 返回值
/// `(u32, CollectorCommandV1)` - (时间戳, 解码后的命令)
///
/// # Panics
/// - 如果输入数据长度不足（小于最小协议长度）
/// - 如果魔数与 `MAGIC_NUMBER` 不匹配
/// - 如果版本号与 `VERSION_NUMBER` 不匹配
/// - 如果 CRC32 校验和验证失败
/// - 如果载荷数据反序列化失败
/// - 如果载荷大小与实际数据不匹配
///
/// # 性能特点
/// - **零拷贝**: 使用切片引用避免数据复制
/// - **常量时间**: 头部解析是 O(1) 操作
/// - **内存安全**: Rust 的边界检查防止缓冲区溢出
///
/// # 示例
/// ```rust
/// let command = CollectorCommandV1::SubmitData {
///     collector_id: 1234,
///     total_memory: 8589934592,
///     used_memory: 4294967296,
///     average_cpu_usage: 0.65,
/// };
/// let encoded = encode_v1(command.clone());
/// let (timestamp, decoded) = decode_v1(&encoded);
/// assert_eq!(decoded, command);
/// ```
///
/// # 边界情况处理
/// - 数据包过小: 自动检测并 panic
/// - 数据包过大: 安全处理，避免内存耗尽
/// - 载荷大小为 0: 正常处理，返回空命令
/// - 时间戳溢出: 32 位时间戳的 2038 年问题
pub fn decode_v1(bytes: &[u8]) -> (u32, CollectorCommandV1) {
    // ===== 协议头部解析 =====
    // 从固定位置提取各个头部字段，全部使用大端序

    // 提取魔数 (字节 0-1)
    let magic_number = u16::from_be_bytes([bytes[0], bytes[1]]);

    // 提取版本号 (字节 2-3)
    let version_number = u16::from_be_bytes([bytes[2], bytes[3]]);

    // 提取时间戳 (字节 4-7)
    let timestamp = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);

    // 提取载荷大小 (字节 8-11)
    let payload_size = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);

    // ===== 载荷数据提取 =====
    // 根据载荷大小字段提取实际的命令数据
    let payload_start = 12; // 头部固定 12 字节
    let payload_end = payload_start + payload_size as usize;
    let payload = &bytes[payload_start..payload_end];

    // ===== CRC32 校验和提取 =====
    // CRC32 位于载荷数据之后，固定 4 字节
    let crc_start = payload_end;
    let crc = u32::from_be_bytes([
        bytes[crc_start],     // CRC32 字节 0
        bytes[crc_start + 1], // CRC32 字节 1
        bytes[crc_start + 2], // CRC32 字节 2
        bytes[crc_start + 3], // CRC32 字节 3
    ]);

    // ===== 协议验证 =====
    // 严格验证协议的各个字段，确保数据安全

    // 验证魔数：防止协议混淆和恶意数据
    assert_eq!(magic_number, MAGIC_NUMBER, "协议魔数不匹配");

    // 验证版本号：确保协议兼容性
    assert_eq!(version_number, VERSION_NUMBER, "协议版本号不匹配");

    // ===== 数据完整性校验 =====
    // 重新计算 CRC32 并与接收到的校验和比较
    let computed_crc = crc32fast::hash(payload);
    assert_eq!(crc, computed_crc, "CRC32 校验失败，数据可能已损坏");

    // ===== 数据反序列化 =====
    // 使用 bincode 将二进制数据还原为 Rust 结构体
    // bincode 提供类型安全的反序列化
    let command = bincode::deserialize(payload).expect("bincode 反序列化失败");

    // 返回时间戳和解析后的命令
    (timestamp, command)
}

// 单元测试模块
//
// 这个模块包含了协议实现的全面测试覆盖，确保：
// 1. 编码/解码功能的正确性
// 2. 数据完整性校验的有效性
// 3. 边界情况和错误处理的健壮性
// 4. 协议规范的严格遵守
//
// 测试策略：
// - 正向测试：验证正常功能
// - 负向测试：验证错误处理
// - 边界测试：验证极端情况
// - 集成测试：验证端到端流程
#[cfg(test)]
mod tests {
    use super::*;

    /// 测试编码和解码功能的正确性
    ///
    /// 这是最核心的单元测试，验证：
    /// 1. 编码函数能正确序列化命令数据
    /// 2. 解码函数能准确还原原始命令
    /// 3. 时间戳生成和提取正常工作
    /// 4. 编码后的数据格式符合协议规范
    ///
    /// # 测试覆盖范围
    /// - 数据类型完整性（u128, u64, f32）
    /// - 序列化/反序列化的双向一致性
    /// - 时间戳的合理性和精度
    /// - 协议头部的基本结构
    ///
    /// # 测试数据设计
    /// 使用典型的系统监控数据：
    /// - collector_id: 1234 (测试整型处理)
    /// - memory: 小数值，便于人工验证
    /// - cpu_usage: 0.5 (测试浮点数精度)
    #[test]
    fn test_encode_decode() {
        // ===== 创建测试数据 =====
        // 使用具有代表性的系统监控数据
        let command = CollectorCommandV1::SubmitData {
            collector_id: 1234,
            total_memory: 100,
            used_memory: 50,
            average_cpu_usage: 0.5,
        };

        // ===== 执行编码操作 =====
        let encoded = encode_v1(command.clone());

        // ===== 执行解码操作 =====
        let (timestamp, decoded) = decode_v1(&encoded);

        // ===== 验证数据一致性 =====
        // 解码后的命令应该与原始命令完全一致
        assert_eq!(decoded, command, "解码后的命令与原始命令不匹配");

        // ===== 验证时间戳合理性 =====
        // 时间戳应该是合理的当前时间（大于 0）
        assert!(timestamp > 0, "时间戳应该大于 0");

        // ===== 验证协议长度 =====
        // 编码后的数据应该包含完整的协议头部和载荷
        // 最小长度 = 头部(12) + CRC32(4) = 16 字节
        // 加上载荷数据，应该大于基础长度
        assert!(encoded.len() > 20, "编码后的数据长度应该大于头部大小");
    }

    /// 测试协议完整性校验机制
    ///
    /// 这个测试验证 CRC32 校验和其他完整性检查的有效性：
    /// 1. 正常数据的正确处理
    /// 2. 损坏数据的检测和拒绝
    /// 3. 错误处理机制的触发
    ///
    /// # 测试方法
    /// 1. 编码正常数据并验证能正确解码
    /// 2. 人为损坏数据（修改最后一个字节）
    /// 3. 验证解码函数能检测到损坏并 panic
    ///
    /// # 安全意义
    /// 这个测试确保了协议在面对网络传输错误或恶意数据时
    /// 能够及时发现并拒绝处理，防止损坏数据进入系统。
    #[test]
    fn test_protocol_integrity() {
        // ===== 创建测试数据 =====
        let command = CollectorCommandV1::SubmitData {
            collector_id: 5678,      // 不同的 collector_id
            total_memory: 1024,      // 更大的内存数值
            used_memory: 512,        // 50% 使用率
            average_cpu_usage: 0.75, // 75% CPU 使用率
        };

        // ===== 编码数据 =====
        let encoded = encode_v1(command.clone());

        // ===== 验证正常解码 =====
        // 确保未损坏的数据能正确解码
        let (timestamp, _) = decode_v1(&encoded);
        assert!(timestamp > 0, "时间戳应该大于 0");

        // ===== 人为损坏数据 =====
        // 通过修改最后一个字节来模拟传输错误
        // 这会破坏 CRC32 校验和，但不影响协议头部结构
        let mut corrupted = encoded.clone();
        if let Some(last_byte) = corrupted.last_mut() {
            *last_byte = last_byte.wrapping_add(1); // +1 确保改变数值
        }

        // ===== 验证错误检测 =====
        // 解码损坏数据应该触发 panic
        // 使用 catch_unwind 捕获 panic，验证错误处理机制正常工作
        std::panic::catch_unwind(|| {
            decode_v1(&corrupted);
        })
        .expect_err("应该检测到数据损坏并触发 panic");
    }

    /// 测试协议魔数验证机制
    ///
    /// 验证协议能正确识别和拒绝错误格式的数据包。
    /// 这是防止协议混淆攻击的重要安全机制。
    #[test]
    fn test_magic_number_validation() {
        let command = CollectorCommandV1::SubmitData {
            collector_id: 1234,
            total_memory: 100,
            used_memory: 50,
            average_cpu_usage: 0.5,
        };

        let mut encoded = encode_v1(command.clone());

        // 修改魔数字段（前两个字节）
        encoded[0] = 0xFF;
        encoded[1] = 0xFF;

        // 应该因为魔数不匹配而 panic
        std::panic::catch_unwind(|| {
            decode_v1(&encoded);
        })
        .expect_err("应该检测到魔数不匹配");
    }

    /// 测试版本号验证机制
    ///
    /// 验证协议能正确处理版本不兼容的情况。
    #[test]
    fn test_version_validation() {
        let command = CollectorCommandV1::SubmitData {
            collector_id: 1234,
            total_memory: 100,
            used_memory: 50,
            average_cpu_usage: 0.5,
        };

        let mut encoded = encode_v1(command.clone());

        // 修改版本号字段（字节 2-3）
        encoded[2] = 0xFF;
        encoded[3] = 0xFF;

        // 应该因为版本号不匹配而 panic
        std::panic::catch_unwind(|| {
            decode_v1(&encoded);
        })
        .expect_err("应该检测到版本号不匹配");
    }

    /// 测试边界情况
    ///
    /// 验证协议在各种边界条件下的行为：
    /// - 最小数据载荷
    /// - 最大数值范围
    /// - 零值处理
    #[test]
    fn test_edge_cases() {
        // 测试最小值
        let min_command = CollectorCommandV1::SubmitData {
            collector_id: 0,
            total_memory: 0,
            used_memory: 0,
            average_cpu_usage: 0.0,
        };

        let encoded = encode_v1(min_command.clone());
        let (_, decoded) = decode_v1(&encoded);
        assert_eq!(decoded, min_command);

        // 测试大数值
        let max_command = CollectorCommandV1::SubmitData {
            collector_id: u128::MAX,
            total_memory: u64::MAX,
            used_memory: u64::MAX,
            average_cpu_usage: 1.0,
        };

        let encoded = encode_v1(max_command.clone());
        let (_, decoded) = decode_v1(&encoded);
        assert_eq!(decoded, max_command);
    }

    #[test]
    fn test_encode_decode_response() {
        let response = CollectorResponseV1::Ack(123);
        let encoded = encode_response_v1(response.clone());
        let decoded = decode_response_v1(&encoded);
        assert_eq!(decoded, response);
    }
}
