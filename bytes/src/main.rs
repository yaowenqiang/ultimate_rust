/*
 * Rust 字节操作和零拷贝深入学习示例
 *
 * 本项目展示了 Rust 中字节操作、类型转换、内存安全和高性能编程的核心概念，
 * 这是理解系统编程、网络编程和数据处理的必备知识。
 *
 * 📚 官方文档链接：
 *
 * 🔰 基础概念
 * 1. Rust Book - 原始类型:
 *    https://doc.rust-lang.org/book/ch03-02-data-types.html
 *
 * 2. Rustonomicon - 转换:
 *    https://doc.rust-lang.org/nomicon/transmutes.html
 *
 * 3. Rust by Example - 指针:
 *    https://doc.rust-lang.org/rust-by-example/std/box.html
 *
 * ⚙️ 字节操作库
 * 4. bytemuck 文档:
 *    https://docs.rs/bytemuck/1.24.0/bytemuck/
 *
 * 5. bytes crate 文档:
 *    https://docs.rs/bytes/1.9.0/bytes/
 *
 * 6. byteorder 文档:
 *    https://docs.rs/byteorder/1.5.0/byteorder/
 *
 * 🚀 高级概念
 * 7. 零拷贝编程:
 *    https://en.wikipedia.org/wiki/Zero-copy
 *
 * 8. 字节序（Endianness）:
 *    https://en.wikipedia.org/wiki/Endianness
 *
 * 9. 内存安全:
 *    https://doc.rust-lang.org/nomicon/meet-safe-and-unsafe.html
 *
 * 🎯 核心学习要点：
 *
 * 🔹 字节和字节数组
 * - 字节是数据的基本单位，8位二进制数
 * - 字节数组是连续的内存块
 * - Rust 提供了安全的字节操作方法
 * - 需要注意对齐和字节序问题
 *
 * 🔹 零拷贝操作
 * - 零拷贝避免不必要的数据复制
 * - 通过类型转换和内存重用实现
 * - bytemuck 提供安全的零拷贝操作
 * - 可以显著提高性能
 *
 * 🔹 字节序处理
 * - 大端序（Big Endian）：网络字节序
 * - 小端序（Little Endian）：x86 架构常用
 * - 需要在不同系统间转换
 * - 影响跨平台数据交换
 *
 * 🔹 类型安全转换
 * - 避免使用裸指针转换
 * - 使用安全的转换库和方法
 * - 注意对齐要求和生命周期
 * - 确保内存安全
 *
 * 🔹 实际应用场景
 * - 网络协议处理
 * - 文件格式解析
 * - 二进制数据处理
 * - 高性能计算
 */

use std::{
    io::Cursor,
    mem,
    str,
    time::Instant,
};

// 导入字节操作相关库
use bytemuck::{Pod, Zeroable, bytes_of, cast_slice, try_cast_slice};
use bytes::{Bytes, BytesMut, Buf, BufMut};
use serde::{Serialize, Deserialize};
use base64::{Engine as _, engine::general_purpose};

// ==================== 1. 基础数据结构定义 ====================

/// 网络包头部结构体 - 展示字节序和内存布局
///
/// 这个结构体模拟了网络协议中常见的包头格式，
/// 展示了如何处理不同大小的数据类型和字节序。
///
/// # 内存布局（大端序）
/// ```
/// +--------+--------+--------+--------+
/// | version| type   | flags  |        |
/// +--------+--------+--------+--------+
/// | length (32位)                   |
/// +---------------------------------+
/// | checksum (32位)                  |
/// +---------------------------------+
/// ```
///
/// # 字段说明
/// * `version` - 协议版本号 (0-255)
/// * `packet_type` - 包类型 (0-255)
/// * `flags` - 标志位 (0-255)
/// * `length` - 数据长度 (32位)
/// * `checksum` - 校验和 (32位)
///
/// # 文档链接
/// - [网络字节序](https://tools.ietf.org/html/rfc1700)
/// - [内存布局](https://doc.rust-lang.org/reference/type-layout.html)
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug, PartialEq)]
struct PacketHeader {
    /// 协议版本号 (1 字节)
    version: u8,
    /// 包类型 (1 字节)
    packet_type: u8,
    /// 标志位 (1 字节)
    flags: u8,
    /// 保留字段 (1 字节)
    reserved: u8,
    /// 数据长度 (4 字节)
    length: u32,
    /// 校验和 (4 字节)
    checksum: u32,
}

/// 3D 点坐标结构体 - 展示浮点数的字节表示
///
/// 浮点数的字节表示在不同架构上可能不同，
/// 需要特别注意在序列化和网络传输中的处理。
///
/// # 字段说明
/// * `x` - X 坐标 (32位浮点数)
/// * `y` - Y 坐标 (32位浮点数)
/// * `z` - Z 坐标 (32位浮点数)
///
/// # 文档链接
/// - [IEEE 754 浮点数标准](https://en.wikipedia.org/wiki/IEEE_754)
/// - [浮点数二进制表示](https://www.h-schmidt.net/FloatConverter/IEEE754.html)
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug, PartialEq)]
struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

/// 学生信息结构体 - 展示混合数据类型的字节布局
///
/// 包含了不同类型的数据，展示了结构体在内存中的布局
/// 和填充字节的处理。
///
/// # 字段说明
/// * `id` - 学生ID (32位整数)
/// * `age` - 年龄 (8位整数)
/// * `gpa` - GPA 成绩 (32位浮点数)
/// * `name_bytes` - 姓名的字节表示 (固定16字节)
///
/// # 文档链接
/// - [结构体布局](https://doc.rust-lang.org/reference/type-layout.html#structs)
/// - [数据对齐](https://en.wikipedia.org/wiki/Data_structure_alignment)
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
struct Student {
    id: u32,
    gpa: f32,
    name_bytes: [u8; 16],
    age: u8,
    _padding: [u8; 3],  // 手动填充对齐
}

/// 带变体的消息枚举 - 展示枚举的字节表示
///
/// 枚举的字节表示取决于其变体和布局属性。
/// 使用 `#[repr(u8)]` 确保枚举使用 1 字节表示。
///
/// # 变体说明
/// * `Ping` - 心跳包，无数据
/// * `Pong` - 心跳响应，无数据
/// * `Data(id)` - 数据包，包含数据ID
/// * `Error(code)` - 错误包，包含错误码
///
/// # 文档链接
/// - [枚举布局](https://doc.rust-lang.org/reference/type-layout.html#enums)
/// - [Rust 枚举优化](https://rust-lang.github.io/unsafe-code-guidelines/layout/enums.html)
#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
enum MessageType {
    Ping = 0,
    Pong = 1,
    Data(u32) = 2,
    Error(u16) = 3,
}

// ==================== 2. 基础字节操作演示 ====================

/// 演示基础字节操作和类型转换
///
/// 这个函数展示了 Rust 中最基本的字节操作，
/// 包括整数到字节数组的转换和字节序处理。
fn demonstrate_basic_byte_operations() {
    println!("🔢 1. 基础字节操作演示:");
    println!("   展示整数到字节数组的转换和字节序处理");

    // 基础整数到字节转换
    let number: u32 = 0x12345678;
    let bytes = number.to_be_bytes();  // 大端序
    let bytes_le = number.to_le_bytes();  // 小端序

    println!("\n   🎯 整数到字节转换:");
    println!("     原始数字: 0x{:08X} ({})", number, number);
    println!("     大端序字节: {:02X?}", bytes);
    println!("     小端序字节: {:02X?}", bytes_le);

    // 字节到整数转换
    let reconstructed = u32::from_be_bytes(bytes);
    let reconstructed_le = u32::from_le_bytes(bytes_le);

    println!("\n   🔄 字节到整数转换:");
    println!("     从大端序重建: 0x{:08X}", reconstructed);
    println!("     从小端序重建: 0x{:08X}", reconstructed_le);
    println!("     转换正确性: {}", reconstructed == number);

    // 浮点数字节操作
    let pi: f32 = std::f32::consts::PI;
    let pi_bytes = pi.to_be_bytes();
    let pi_reconstructed = f32::from_be_bytes(pi_bytes);

    println!("\n   📊 浮点数字节操作:");
    println!("     PI 值: {}", pi);
    println!("     PI 字节: {:02X?}", pi_bytes);
    println!("     重建 PI: {}", pi_reconstructed);
    println!("     精度保持: {}", (pi - pi_reconstructed).abs() < f32::EPSILON);

    // 字符串字节操作
    let text = "Hello, 世界!";
    let text_bytes = text.as_bytes();
    let text_from_bytes = str::from_utf8(text_bytes).unwrap();

    println!("\n   🔤 字符串字节操作:");
    println!("     原始文本: {}", text);
    println!("     UTF-8 字节: {:02X?}", text_bytes);
    println!("     字节长度: {} 字节", text_bytes.len());
    println!("     重建文本: {}", text_from_bytes);
    println!("     UTF-8 有效: {}", text_from_bytes.is_char_boundary(text_bytes.len()));

    println!();
}

/// 演示字节序转换和跨平台兼容性
fn demonstrate_endianness() {
    println!("🔄 2. 字节序转换演示:");
    println!("   展示大端序和小端序的转换和平台差异");

    let test_values = [
        0x12345678u32,
        0xABCDEF00u32,
        0x00000001u32,
        0xFFFFFFFFu32,
    ];

    println!("\n   📊 字节序对比表:");
    println!("     {:<12} {:<20} {:<20} {:<10}",
             "数值", "大端序 (BE)", "小端序 (LE)", "系统序");
    println!("     {:<12} {:<20} {:<20} {:<10}",
             "----", "--------", "--------", "------");

    for &value in &test_values {
        let be_bytes = value.to_be_bytes();
        let le_bytes = value.to_le_bytes();
        let native_bytes = value.to_ne_bytes();  // 本机字节序

        println!("     0x{:08X}   {:<20} {:<20} {:<10}",
                 value,
                 format!("{:02X?}", be_bytes),
                 format!("{:02X?}", le_bytes),
                 format!("{:02X?}", native_bytes));
    }

    // 检测当前系统的字节序
    let test_value = 0x12345678u32;
    let is_little_endian = test_value.to_le_bytes() == test_value.to_ne_bytes();

    println!("\n   💻 系统信息:");
    println!("     当前系统字节序: {}",
             if is_little_endian { "小端序 (LE)" } else { "大端序 (BE)" });
    println!("     网络字节序标准: 大端序");
    println!("     x86/x64 架构: 小端序");

    // 网络字节序转换示例
    let host_value: u32 = 0x12345678;
    let network_value = host_value.to_be();  // 转换为网络字节序
    let host_converted = u32::from_be(network_value);  // 从网络字节序转换回来

    println!("\n   🌐 网络字节序转换:");
    println!("     主机字节序: 0x{:08X}", host_value);
    println!("     网络字节序: 0x{:08X}", network_value);
    println!("     转换回来:   0x{:08X}", host_converted);
    println!("     转换正确:   {}", host_value == host_converted);

    println!();
}

// ==================== 3. bytemuck 零拷贝操作演示 ====================

/// 演示 bytemuck 库的零拷贝字节操作
fn demonstrate_bytemuck_operations() {
    println!("⚡ 3. bytemuck 零拷贝操作演示:");
    println!("   展示安全的零拷贝类型转换和内存操作");

    // 创建测试数据
    let points = [
        Point3D { x: 1.0, y: 2.0, z: 3.0 },
        Point3D { x: 4.0, y: 5.0, z: 6.0 },
        Point3D { x: 7.0, y: 8.0, z: 9.0 },
    ];

    println!("\n   🎯 原始数据:");
    for (i, point) in points.iter().enumerate() {
        println!("     Point[{}]: ({:.1}, {:.1}, {:.1})", i, point.x, point.y, point.z);
    }

    // 零拷贝转换为字节数组
    let point_bytes = bytes_of(&points);
    println!("\n   🔄 零拷贝转换:");
    println!("     结构体数量: {}", points.len());
    println!("     每个大小: {} 字节", mem::size_of::<Point3D>());
    println!("     字节总数: {} 字节", point_bytes.len());
    println!("     字节预览: {:02X?}", &point_bytes[..16]);

    // 零拷贝从字节重建结构体
    let reconstructed_points: &[Point3D] = cast_slice(point_bytes);
    println!("\n   🔙 从字节重建:");
    println!("     重建数量: {}", reconstructed_points.len());
    println!("     数据一致性: {}", points == *reconstructed_points);

    // 验证数据正确性
    println!("\n   ✅ 数据验证:");
    for (i, (original, reconstructed)) in points.iter().zip(reconstructed_points.iter()).enumerate() {
        println!("     Point[{}]: 原=({:.1}, {:.1}, {:.1}) 重=({:.1}, {:.1}, {:.1}) {}",
                 i, original.x, original.y, original.z,
                 reconstructed.x, reconstructed.y, reconstructed.z,
                 if original == reconstructed { "✓" } else { "✗" });
    }

    // 演示不安全转换的检测
    println!("\n   🛡️  安全检查:");
    let mismatched_bytes = &[0u8; 10];  // 长度不匹配的字节数组
    let safe_result = try_cast_slice::<u8, Point3D>(mismatched_bytes);
    match safe_result {
        Ok(_) => println!("     转换成功 (不应该发生)"),
        Err(e) => println!("     安全检查失败: {}", e),
    }

    println!();
}

/// 演示复杂的结构体零拷贝操作
fn demonstrate_complex_struct_operations() {
    println!("🏗️  4. 复杂结构体零拷贝操作演示:");
    println!("   展示包含填充字节和对齐的结构体操作");

    // 创建学生数据
    let students = [
        Student {
            id: 1001,
            gpa: 3.8,
            name_bytes: {
                let mut name = [0u8; 16];
                name[..2].copy_from_slice(b"ZS");
                name
            },
            age: 20,
            _padding: [0, 0, 0],
        },
        Student {
            id: 1002,
            gpa: 3.6,
            name_bytes: {
                let mut name = [0u8; 16];
                name[..2].copy_from_slice(b"LS");
                name
            },
            age: 21,
            _padding: [0, 0, 0],
        },
    ];

    println!("\n   👨‍🎓 学生数据:");
    for (i, student) in students.iter().enumerate() {
        let name_str = std::str::from_utf8(&student.name_bytes)
            .unwrap_or("Invalid UTF-8")
            .trim_end_matches('\0');
        println!("     学生[{}]: ID={}, 年龄={}, GPA={:.1}, 姓名={}",
                 i, student.id, student.age, student.gpa, name_str);
    }

    // 分析内存布局
    println!("\n   📊 内存布局分析:");
    println!("     Student 结构体大小: {} 字节", mem::size_of::<Student>());
    println!("     对齐要求: {} 字节", mem::align_of::<Student>());
    println!("     字段数量: 4 个");

    // 零拷贝转换
    let student_bytes = bytes_of(&students);
    println!("\n   🔄 零拷贝操作:");
    println!("     学生数量: {}", students.len());
    println!("     字节总数: {} 字节", student_bytes.len());
    println!("     每学生占用: {} 字节", mem::size_of::<Student>());

    // 显示内存布局细节
    println!("\n   🧠 内存布局详情:");
    for (i, student) in students.iter().enumerate() {
        let offset = i * mem::size_of::<Student>();
        let student_bytes = &student_bytes[offset..offset + mem::size_of::<Student>()];
        println!("     学生[{}] 字节: {:02X?}", i, student_bytes);

        // 解析各个字段
        let id_bytes = &student_bytes[0..4];
        let gpa_bytes = &student_bytes[4..8];
        let name_bytes = &student_bytes[8..24];
        let age_byte = student_bytes[24];

        println!("       ID: {:02X?} -> {}", id_bytes, student.id);
        println!("       GPA: {:02X?} -> {:.1}", gpa_bytes, student.gpa);
        println!("       姓名: {:02X?}", name_bytes);
        println!("       年龄: {:02X} -> {}", age_byte, student.age);
    }

    println!();
}

// ==================== 4. bytes crate 高级操作演示 ====================

/// 演示 bytes crate 的高级字节操作
fn demonstrate_bytes_crate_operations() {
    println!("📦 5. bytes crate 高级操作演示:");
    println!("   展示高性能字节缓冲区和引用计数");

    // 创建 BytesMut 用于写入
    let mut buffer = BytesMut::new();

    println!("\n   ✏️  写入操作:");
    println!("     初始容量: {} 字节", buffer.capacity());

    // 写入不同类型的数据
    buffer.put_u8(0x12);
    buffer.put_u16(0x3456);
    buffer.put_u32(0x789ABCDE);
    buffer.put_f32(3.14159);
    buffer.put_slice(b"Hello, Rust!");

    println!("     写入后容量: {} 字节", buffer.capacity());
    println!("     数据长度: {} 字节", buffer.len());
    println!("     缓冲区内容: {:02X?}", &buffer[..]);

    // 创建 Bytes 用于读取和共享
    let bytes = buffer.freeze();

    println!("\n   📖 读取操作:");
    println!("     Bytes 引用计数: {:?}", bytes.as_ptr());

    // 模拟读取数据
    let mut cursor = Cursor::new(bytes.clone());
    let byte1 = cursor.get_u8();
    let word1 = cursor.get_u16();
    let dword1 = cursor.get_u32();
    let float1 = cursor.get_f32();
    let remaining = cursor.remaining();
    let text_bytes = cursor.copy_to_bytes(remaining);

    println!("     读取 u8: 0x{:02X}", byte1);
    println!("     读取 u16: 0x{:04X}", word1);
    println!("     读取 u32: 0x{:08X}", dword1);
    println!("     读取 f32: {}", float1);
    println!("     剩余文本: {:?}", str::from_utf8(&text_bytes).unwrap());

    // 演示零拷贝切片
    println!("\n   🔪 零拷贝切片:");
    let slice1 = bytes.slice(0..4);    // 前4字节
    let slice2 = bytes.slice(4..12);   // 中间8字节
    let slice3 = bytes.slice(12..);    // 剩余字节

    println!("     切片1 (0-4): {:02X?}", &slice1);
    println!("     切片2 (4-12): {:02X?}", &slice2);
    println!("     切片3 (12-): {:02X?}", &slice3);
    println!("     所有切片共享同一块内存: 是");

    println!();
}

/// 演示异步字节操作（使用 tokio）
async fn demonstrate_async_byte_operations() {
    println!("⚡ 6. 异步字节操作演示:");
    println!("   展示异步环境下的字节操作和网络编程");

    // 模拟异步数据源
    let data_source = vec![
        PacketHeader {
            version: 1,
            packet_type: 0x01,
            flags: 0x80,
            reserved: 0,
            length: 12,
            checksum: 0x12345678,
        },
        PacketHeader {
            version: 1,
            packet_type: 0x02,
            flags: 0x40,
            reserved: 0,
            length: 8,
            checksum: 0x87654321,
        },
    ];

    println!("\n   📡 模拟网络数据包:");
    for (i, packet) in data_source.iter().enumerate() {
        println!("     包[{}]: 版本={}, 类型={}, 标志=0x{:02X}, 长度={}",
                 i, packet.version, packet.packet_type, packet.flags, packet.length);
    }

    // 异步序列化
    let mut buffer = BytesMut::new();
    for packet in &data_source {
        buffer.extend_from_slice(bytes_of(packet));
    }

    println!("\n   📦 序列化结果:");
    println!("     包数量: {}", data_source.len());
    println!("     总字节: {}", buffer.len());
    println!("     数据: {:02X?}", &buffer[..std::cmp::min(32, buffer.len())]);

    // 模拟异步解析
    let frozen_buffer = buffer.freeze();
    let packets = parse_packets_async(&frozen_buffer).await;

    println!("\n   🔄 异步解析结果:");
    println!("     解析包数: {}", packets.len());
    for (i, packet) in packets.iter().enumerate() {
        println!("     包[{}]: {:?}", i, packet);
    }

    println!();
}

/// 异步解析网络包的辅助函数
async fn parse_packets_async(buffer: &Bytes) -> Vec<PacketHeader> {
    // 模拟异步延迟
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    let header_size = mem::size_of::<PacketHeader>();
    let packet_count = buffer.len() / header_size;
    let mut packets = Vec::new();

    for i in 0..packet_count {
        let offset = i * header_size;
        if offset + header_size <= buffer.len() {
            let packet_bytes = &buffer[offset..offset + header_size];
            if let Ok(packet_slice) = try_cast_slice::<u8, PacketHeader>(packet_bytes) {
                packets.push(packet_slice[0]);
            }
        }
    }

    packets
}

// ==================== 5. 编码和解码操作演示 ====================

/// 演示各种编码和解码操作
fn demonstrate_encoding_decoding() {
    println!("🔐 7. 编码和解码操作演示:");
    println!("   展示十六进制、Base64 等编码方式");

    let original_data = "Hello, Rust! 你好，Rust！".as_bytes();

    println!("\n   📝 原始数据:");
    println!("     文本: {}", str::from_utf8(original_data).unwrap());
    println!("     字节: {:02X?}", original_data);
    println!("     长度: {} 字节", original_data.len());

    // 十六进制编码/解码
    let hex_encoded = hex::encode(original_data);
    let hex_decoded = hex::decode(&hex_encoded).unwrap();

    println!("\n   🔢 十六进制编码:");
    println!("     编码结果: {}", hex_encoded);
    println!("     编码长度: {} 字符", hex_encoded.len());
    println!("     解码正确: {}", hex_decoded == original_data);

    // Base64 编码/解码
    let base64_encoded = general_purpose::STANDARD.encode(original_data);
    let base64_decoded = general_purpose::STANDARD.decode(&base64_encoded).unwrap();

    println!("\n   📋 Base64 编码:");
    println!("     编码结果: {}", base64_encoded);
    println!("     编码长度: {} 字符", base64_encoded.len());
    println!("     解码正确: {}", base64_decoded == original_data);

    // URL 安全的 Base64
    let url_safe_encoded = general_purpose::URL_SAFE_NO_PAD.encode(original_data);
    let url_safe_decoded = general_purpose::URL_SAFE_NO_PAD.decode(&url_safe_encoded).unwrap();

    println!("\n   🌐 URL安全 Base64:");
    println!("     编码结果: {}", url_safe_encoded);
    println!("     解码正确: {}", url_safe_decoded == original_data);

    // 编码效率对比
    println!("\n   📊 编码效率对比:");
    println!("     原始长度: {} 字节", original_data.len());
    println!("     十六进制: {} 字符 ({}%)",
             hex_encoded.len(),
             hex_encoded.len() as f64 / original_data.len() as f64 * 100.0);
    println!("     Base64: {} 字符 ({}%)",
             base64_encoded.len(),
             base64_encoded.len() as f64 / original_data.len() as f64 * 100.0);

    println!();
}

/// 演示序列化框架的字节操作
fn demonstrate_serialization_byte_operations() {
    println!("📦 8. 序列化框架字节操作演示:");
    println!("   展示 serde、JSON 和二进制序列化的字节处理");

    // 创建测试数据
    let test_data = TestMessage {
        id: 12345,
        message_type: MessageType::Data(67890),
        timestamp: 1634567890,
        payload: vec![1, 2, 3, 4, 5],
        metadata: Some("测试元数据".to_string()),
    };

    println!("\n   📋 测试数据:");
    println!("     ID: {}", test_data.id);
    println!("     类型: {:?}", test_data.message_type);
    println!("     时间戳: {}", test_data.timestamp);
    println!("     载荷长度: {}", test_data.payload.len());
    println!("     元数据: {:?}", test_data.metadata);

    // JSON 序列化
    let json_str = serde_json::to_string(&test_data).unwrap();
    let json_bytes = json_str.as_bytes();

    println!("\n   📄 JSON 序列化:");
    println!("     字符串长度: {} 字符", json_str.len());
    println!("     字节长度: {} 字节", json_bytes.len());
    println!("     内容: {}", json_str);

    // 二进制序列化 (bincode)
    let binary_data = bincode::serialize(&test_data).unwrap();

    println!("\n   🔢 二进制序列化:");
    println!("     字节长度: {} 字节", binary_data.len());
    println!("     字节内容: {:02X?}", &binary_data[..std::cmp::min(32, binary_data.len())]);

    // 反序列化验证
    let json_restored: TestMessage = serde_json::from_str(&json_str).unwrap();
    let binary_restored: TestMessage = bincode::deserialize(&binary_data).unwrap();

    println!("\n   ✅ 反序列化验证:");
    println!("     JSON 还原正确: {}", json_restored == test_data);
    println!("     二进制还原正确: {}", binary_restored == test_data);

    // 压缩比对比
    let original_size = mem::size_of::<TestMessage>();

    println!("\n   📊 大小对比:");
    println!("     内存中大小: {} 字节", original_size);
    println!("     JSON 大小: {} 字节 ({:.1}x)",
             json_bytes.len(),
             json_bytes.len() as f64 / original_size as f64);
    println!("     二进制大小: {} 字节 ({:.1}x)",
             binary_data.len(),
             binary_data.len() as f64 / original_size as f64);

    println!();
}

/// 测试消息结构体 - 用于序列化演示
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct TestMessage {
    id: u32,
    message_type: MessageType,
    timestamp: u64,
    payload: Vec<u8>,
    metadata: Option<String>,
}

// ==================== 6. 性能分析和优化演示 ====================

/// 演示字节操作的性能分析和优化
fn demonstrate_performance_analysis() {
    println!("⚡ 9. 性能分析和优化演示:");
    println!("   对比不同字节操作方法的性能");

    const ITERATIONS: usize = 1_000;
    const DATA_SIZE: usize = 100_000;  // 100KB 数据

    // 生成测试数据
    let test_data: Vec<u8> = (0..255).cycle().take(DATA_SIZE).collect();
    let test_points: Vec<Point3D> = (0..(DATA_SIZE / mem::size_of::<Point3D>()))
        .map(|i| Point3D {
            x: (i % 1000) as f32,
            y: ((i / 1000) % 1000) as f32,
            z: (i / 1_000_000) as f32
        })
        .collect();

    println!("\n   📊 测试配置:");
    println!("     迭代次数: {}", ITERATIONS);
    println!("     数据大小: {:.2} MB", DATA_SIZE as f64 / 1024.0 / 1024.0);
    println!("     点数量: {}", test_points.len());

    // 测试1: 传统复制方法
    println!("\n   1️⃣  传统复制方法:");
    let start = Instant::now();
    let mut sum = 0u32;

    for _ in 0..ITERATIONS {
        for &byte in &test_data {
            sum = sum.wrapping_add(byte as u32);
        }
    }

    let copy_time = start.elapsed();
    println!("     时间: {:?}", copy_time);
    println!("     结果: {}", sum);

    // 测试2: bytemuck 零拷贝方法
    println!("\n   2️⃣  bytemuck 零拷贝方法:");
    let start = Instant::now();
    let mut sum_x = 0.0f32;

    for _ in 0..ITERATIONS {
        for point in &test_points {
            sum_x += point.x;
        }
    }

    let zero_copy_time = start.elapsed();
    println!("     时间: {:?}", zero_copy_time);
    println!("     X坐标总和: {:.2}", sum_x);

    // 测试3: bytes crate 方法
    println!("\n   3️⃣  bytes crate 方法:");
    let start = Instant::now();
    let mut buffer_sum = 0u64;

    for _ in 0..ITERATIONS {
        let buffer = Bytes::copy_from_slice(&test_data);
        for byte in buffer.iter() {
            buffer_sum = buffer_sum.wrapping_add(*byte as u64);
        }
    }

    let bytes_time = start.elapsed();
    println!("     时间: {:?}", bytes_time);
    println!("     结果: {}", buffer_sum);

    // 性能对比
    println!("\n   📈 性能对比:");
    println!("     传统复制: {:?}", copy_time);
    println!("     零拷贝: {:?}", zero_copy_time);
    println!("     bytes crate: {:?}", bytes_time);

    if copy_time < zero_copy_time && copy_time < bytes_time {
        println!("     🏆 最快: 传统复制方法");
    } else if zero_copy_time < bytes_time {
        println!("     🏆 最快: 零拷贝方法");
    } else {
        println!("     🏆 最快: bytes crate 方法");
    }

    // 内存使用分析
    println!("\n   💾 内存使用分析:");
    println!("     Vec<u8> 内存开销: {:.2} MB",
             (test_data.capacity() * mem::size_of::<u8>()) as f64 / 1024.0 / 1024.0);
    println!("     Bytes 引用计数开销: 最小");
    println!("     bytemuck 零拷贝开销: 零");

    println!();
}

// ==================== 主函数 ====================

fn main() {
    println!("=== Rust 字节操作和零拷贝深入学习示例 ===\n");

    println!("本示例将演示字节操作、类型转换、零拷贝编程和性能优化，");
    println!("这是理解系统编程、网络编程和数据处理的核心概念。\n");

    println!("🚀 开始学习之旅...\n");

    // 1. 基础字节操作
    demonstrate_basic_byte_operations();

    // 2. 字节序转换
    demonstrate_endianness();

    // 3. bytemuck 零拷贝操作
    demonstrate_bytemuck_operations();

    // 4. 复杂结构体操作
    demonstrate_complex_struct_operations();

    // 5. bytes crate 高级操作
    demonstrate_bytes_crate_operations();

    // 6. 异步字节操作 (在 tokio 运行时中)
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(demonstrate_async_byte_operations());

    // 7. 编码和解码
    demonstrate_encoding_decoding();

    // 8. 序列化框架
    demonstrate_serialization_byte_operations();

    // 9. 性能分析
    demonstrate_performance_analysis();

    println!("=== 字节操作和零拷贝学习总结 ===");
    println!("🎯 核心概念回顾:");
    println!("  • 字节是数据处理的基本单位");
    println!("  • 零拷贝可以显著提高性能");
    println!("  • 字节序影响跨平台数据交换");
    println!("  • 安全的类型转换避免内存错误");
    println!();
    println!("💡 最佳实践:");
    println!("  • 优先使用安全的转换方法");
    println!("  • 注意对齐和字节序问题");
    println!("  • 选择合适的编码方式");
    println!("  • 进行性能测试验证优化");
    println!();
    println!("🔧 实际应用:");
    println!("  • 网络协议处理和解析");
    println!("  • 文件格式读写");
    println!("  • 数据序列化和传输");
    println!("  • 高性能数据处理");
    println!();
    println!("✅ 学习完成！您已经掌握了 Rust 字节操作的核心概念。");
}