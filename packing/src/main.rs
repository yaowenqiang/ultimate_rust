/*
 * Rust 内存打包和对齐深入学习示例
 *
 * 本项目展示了 Rust 中内存打包（Memory Packing）、对齐（Alignment）和布局（Layout）的概念，
 * 这是理解 Rust 内存布局、性能优化和系统编程的关键概念。
 *
 * 📚 官方文档链接：
 *
 * 🔰 基础概念
 * 1. Rust Book - 类型布局:
 *    https://doc.rust-lang.org/reference/type-layout.html
 *
 * 2. Rustonomicon - 数据布局:
 *    https://doc.rust-lang.org/nomicon/data.html
 *
 * 3. Rust by Example - 属性:
 *    https://doc.rust-lang.org/rust-by-example/attribute.html
 *
 * ⚙️ 属性文档
 * 4. repr 属性文档:
 *    https://doc.rust-lang.org/reference/type-layout.html#reprc-enums
 *
 * 5. packed 属性:
 *    https://doc.rust-lang.org/reference/type-layout.html#reprpacked
 *
 * 6. aligned 属性:
 *    https://doc.rust-lang.org/reference/type-layout.html#repraligned
 *
 * 🚀 高级概念
 * 7. 内存对齐概念:
 *    https://en.wikipedia.org/wiki/Data_structure_alignment
 *
 * 8. 字节序（Endianness）:
 *    https://en.wikipedia.org/wiki/Endianness
 *
 * 9. 网络字节序:
 *    https://tools.ietf.org/html/rfc1700
 *
 * 🎯 核心学习要点：
 *
 * 🔹 内存对齐的本质
 * - 内存对齐是数据在内存中的排列方式
 * - 对齐要求：数据地址必须是其大小的整数倍
 * - 对齐可以提高内存访问性能
 * - 错误的对齐可能导致性能下降或程序崩溃
 *
 * 🔹 内存打包的概念
 * - 内存打包是减少内存占用的技术
 * - packed 属性可以移除填充字节
 * - 但可能影响性能和可移植性
 * - 需要在性能和空间之间权衡
 *
 * 🔹 布局控制
 * - repr(C) - C 兼容布局
 * - repr(packed) - 紧凑布局
 * - repr(align(n)) - 指定对齐
 * - repr(transparent) - 透明包装
 *
 * 🔹 实际应用场景
 * - 网络协议数据处理
 * - 文件格式解析
 * - 嵌入式系统编程
 * - 高性能计算
 */

use std::{
    mem::{align_of, size_of},
    fmt::Debug,
};
use serde::{Serialize, Deserialize};
use bitflags::bitflags;

// ==================== 1. 基础内存布局示例 ====================

/// 单字节结构体 - 最简单的内存布局
///
/// 这个结构体只包含一个 u8 字段，展示了最基本的内存布局。
/// 由于 u8 的对齐要求是 1，所以这个结构体不需要任何填充。
///
/// # 内存布局
/// ```
/// +-------+
/// | a: u8 |  // 1 字节
/// +-------+
/// 总大小: 1 字节，对齐: 1 字节
/// ```
///
/// # 字段说明
/// * `a` - 8位无符号整数，占 1 字节
///
/// # 文档链接
/// - [基本数据类型](https://doc.rust-lang.org/reference/types/numeric.html)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct OneByte {
    a: u8,
}

/// 双字节结构体 - 展示自然对齐
///
/// 这个结构体包含一个 u16 字段，展示了 2 字节对齐的概念。
/// u16 的对齐要求是 2，意味着这个结构体的地址必须是 2 的倍数。
///
/// # 内存布局
/// ```
/// +--------+
/// | a: u16 |  // 2 字节，地址必须对齐到 2 字节边界
/// +--------+
/// 总大小: 2 字节，对齐: 2 字节
/// ```
///
/// # 字段说明
/// * `a` - 16位无符号整数，占 2 字节
///
/// # 文档链接
/// - [数值类型对齐](https://doc.rust-lang.org/reference/type-layout.html#primitive-data-layout)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct TwoByte {
    a: u16,
}

/// 三字节结构体 - 展示非对齐大小的影响
///
/// 这个结构体包含一个 u16 和一个 u8，总大小为 3 字节。
/// 但由于内存对齐要求，实际大小可能大于 3 字节。
///
/// # 原始布局（理论）
/// ```
/// +--------+-------+
/// | a: u16 | b: u8 |  // 3 字节
/// +--------+-------+
/// ```
///
/// # 实际布局（对齐后）
/// ```
/// +--------+-------+-------+
/// | a: u16 | b: u8 | padding|  // 4 字节，填充 1 字节
/// +--------+-------+-------+
/// ```
///
/// # 字段说明
/// * `a` - 16位无符号整数，占 2 字节
/// * `b` - 8位无符号整数，占 1 字节
///
/// # 文档链接
/// - [结构体对齐](https://doc.rust-lang.org/reference/type-layout.html#structs)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct ThreeByte {
    a: u16,
    b: u8,
}

/// 三字节紧凑结构体 - 展示 packed 属性的使用
///
/// 使用 #[repr(packed)] 属性移除所有填充字节，使结构体大小精确等于字段大小之和。
///
/// ⚠️ **警告**: 使用 packed 可能导致性能问题和未定义行为
/// - 访问未对齐的字段可能导致性能下降
/// - 在某些架构上可能引发硬件异常
/// - 应谨慎使用，仅在必要时采用
///
/// # 内存布局（packed）
/// ```
/// +--------+-------+
/// | a: u16 | b: u8 |  // 3 字节，无填充
/// +--------+-------+
/// 总大小: 3 字节，对齐: 1 字节（packed 强制）
/// ```
///
/// # 字段说明
/// * `a` - 16位无符号整数，可能未对齐
/// * `b` - 8位无符号整数
///
/// # 文档链接
/// - [packed 属性](https://doc.rust-lang.org/reference/type-layout.html#reprpacked)
/// - [未对齐访问](https://doc.rust-lang.org/nomicon/uninitialized.html)
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
struct ThreeBytePacked {
    a: u16,
    b: u8,
}

/// 四字节结构体 - 展示 32 位整数对齐
///
/// 这个结构体包含一个 u32 字段，展示了 4 字节对齐。
/// u32 的对齐要求通常是 4（取决于目标架构）。
///
/// # 内存布局
/// ```
/// +------------+
/// | a: u32     |  // 4 字节，对齐到 4 字节边界
/// +------------+
/// 总大小: 4 字节，对齐: 4 字节
/// ```
///
/// # 字段说明
/// * `a` - 32位无符号整数，占 4 字节
///
/// # 文档链接
/// - [32位整数对齐](https://doc.rust-lang.org/reference/type-layout.html#primitive-data-layout)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct FourByte {
    a: u32,
}

/// 混合类型结构体 - 展示复杂内存布局
///
/// 这个结构体包含多种类型，展示了 Rust 如何自动处理内存对齐。
///
/// # 内存布局分析
/// ```
/// 字段顺序: u8 -> u32 -> u16 -> u8
/// 重新排序后: u32 -> u16 -> u8 -> u8
///
/// 实际布局（64位系统）:
/// +------------+--------+-------+-------+-------+
/// | a: u32     | b: u16 | c: u8 | d: u8 | pad1  |  // 8 字节
/// +------------+--------+-------+-------+-------+
/// ```
///
/// # 字段说明
/// * `a` - 32位无符号整数，对齐 4 字节
/// * `b` - 16位无符号整数，对齐 2 字节
/// * `c` - 8位无符号整数，对齐 1 字节
/// * `d` - 8位无符号整数，对齐 1 字节
///
/// # 文档链接
/// - [复杂结构体布局](https://doc.rust-lang.org/reference/type-layout.html#structs)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct MixedType {
    a: u32,
    b: u16,
    c: u8,
    d: u8,
}

/// 嵌套结构体 - 展示嵌套类型的内存布局
///
/// 这个结构体包含其他结构体作为字段，展示了嵌套类型的内存对齐规则。
///
/// # 内存布局
/// ```
/// Outer 结构体布局:
/// +----------------+----------------+
/// | inner: Inner   | value: u32     |  // 8 字节 + 4 字节 + 填充
/// +----------------+----------------+
///
/// Inner 结构体布局:
/// +--------+--------+
/// | a: u16 | b: u8 | padding
/// +--------+--------+
/// ```
///
/// # 字段说明
/// * `inner` - 嵌套的 Inner 结构体
/// * `value` - 32位无符号整数
///
/// # 文档链接
/// - [嵌套结构体布局](https://doc.rust-lang.org/reference/type-layout.html#structs)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Outer {
    inner: Inner,
    value: u32,
}

/// 内部结构体 - 用于嵌套测试
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Inner {
    a: u16,
    b: u8,
}

// ==================== 2. 对齐控制示例 ====================

/// 自定义对齐结构体 - 展示 align 属性的使用
///
/// 使用 #[repr(align(n))] 属性指定结构体的对齐要求。
/// 这可以用于特殊用途，如 SIMD 操作或硬件接口。
///
/// # 内存布局
/// ```
/// 对齐要求: 16 字节
/// +--------+
/// | a: u32 |  // 4 字节
/// +--------+
// | padding|  // 12 字节填充
/// +--------+
/// 总大小: 16 字节（对齐到 16 字节边界）
/// ```
///
/// # 字段说明
/// * `a` - 32位无符号整数
///
/// # 文档链接
/// - [align 属性](https://doc.rust-lang.org/reference/type-layout.html#repraligned)
#[repr(align(16))]
#[derive(Debug, Clone, Copy)]
struct Aligned16 {
    a: u32,
}

/// 透明包装结构体 - 展示 transparent 属性
///
/// 使用 #[repr(transparent)] 属性使包装结构体具有与内部类型相同的布局。
/// 这对于零成本抽象和类型安全的包装很有用。
///
/// # 内存布局
/// ```
/// Wrapper<T> 和 T 具有相同的布局和 ABI
/// ```
///
/// # 泛型参数
/// * `T` - 被包装的类型
///
/// # 字段说明
/// * `value` - 被包装的值
///
/// # 文档链接
/// - [transparent 属性](https://doc.rust-lang.org/reference/type-layout.html#reprtransparent)
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
struct Wrapper<T: Copy + Debug> {
    value: T,
}

// ==================== 3. 联合体和枚举布局示例 ====================

/// 联合体 - 展示内存重叠的概念
///
/// 联合体的所有字段共享同一块内存，大小等于最大字段的大小。
/// 这在需要类型转换或节省内存时很有用。
///
/// # 内存布局
/// ```
/// +------------+
/// | data: u32  |  // 4 字节
/// | parts      |  // 同样的 4 字节，重叠存储
/// +------------+
/// 总大小: 4 字节（max(4, 4)）
/// ```
///
/// # 字段说明
/// * `data` - 32位完整数据
/// * `parts` - 两个16位部分，与 data 重叠
///
/// # 文档链接
/// - [联合体](https://doc.rust-lang.org/reference/items/unions.html)
#[repr(C)]
union DataUnion {
    data: u32,
    parts: Parts,
}

impl Debug for DataUnion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            write!(f, "DataUnion {{ data: 0x{:08X} }}", self.data)
        }
    }
}

/// 部分数据结构 - 用于联合体
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Parts {
    low: u16,
    high: u16,
}

/// 枚举 - 展示不同大小的枚举布局
///
/// 枚举的内存布局取决于其变体和字段。
/// 无字段的枚举通常使用最小的整数类型。
///
/// # 字段说明
/// * `None` - 空变体
/// * `Some(value)` - 包含值的变体
///
/// # 文档链接
/// - [枚举内存布局](https://doc.rust-lang.org/reference/type-layout.html#enums)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
enum OptionEnum {
    None,
    Some(u32),
}

// ==================== 4. 字节序和序列化示例 ====================

/// 网络包结构体 - 展示字节序处理
///
/// 这个结构体展示了网络编程中常见的内存布局问题。
/// 网络协议通常要求特定的字节序（大端序）。
///
/// # 字段说明
/// * `magic` - 魔数，用于识别协议
/// * `version` - 协议版本
/// * `length` - 数据长度
/// * `checksum` - 校验和
///
/// # 文档链接
/// - [网络字节序](https://tools.ietf.org/html/rfc1700)
/// - [字节序处理](https://doc.rust-lang.org/std/num/index.html#big-endian-byte-order)
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]  // 使用 packed 确保网络包格式
struct NetworkPacket {
    magic: u32,
    version: u16,
    length: u16,
    checksum: u32,
}

/// 可序列化的数据结构 - 展示序列化布局
///
/// 使用 serde 框架演示序列化时的内存布局考虑。
///
/// # 字段说明
/// * `id` - 数据 ID
/// * `name` - 数据名称
/// * `timestamp` - 时间戳
/// * `data` - 数据内容
///
/// # 文档链接
/// - [Serde 文档](https://serde.rs/)
/// - [序列化概念](https://doc.rust-lang.org/std/num/index.html#big-endian-byte-order)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SerializableData {
    id: u64,
    name: String,
    timestamp: u32,
    data: Vec<u8>,
}

// ==================== 5. 位字段示例 ====================

bitflags! {
    /// 权限标志位 - 展示位操作和内存打包
    ///
    /// 使用 bitflags 宏来创建类型安全的位标志。
    /// 这在需要节省内存或进行位操作时很有用。
    ///
    /// # 字段说明
    /// 每个标志占用一个位，可以组合使用
    ///
    /// # 文档链接
    /// - [bitflags 文档](https://docs.rs/bitflags/)
    /// - [位操作](https://doc.rust-lang.org/std/ops/index.html#bit-operations)
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FilePermissions: u8 {
        /// 读取权限
        const READ    = 0b0000_0001;
        /// 写入权限
        const WRITE   = 0b0000_0010;
        /// 执行权限
        const EXECUTE = 0b0000_0100;
        /// 删除权限
        const DELETE  = 0b0000_1000;
        /// 所有权限
        const ALL = Self::READ.bits() | Self::WRITE.bits() | Self::EXECUTE.bits() | Self::DELETE.bits();
    }
}

// ==================== 6. 演示函数 ====================

/// 演示基础内存布局
///
/// 这个函数展示了不同大小结构体的内存布局，
/// 帮助理解 Rust 的默认内存对齐规则。
fn demonstrate_basic_layout() {
    println!("🔢 1. 基础内存布局演示:");
    println!("   展示不同大小结构体的内存占用和对齐");

    // 基础类型大小
    println!("   基础类型大小:");
    println!("     u8  : {} 字节", size_of::<u8>());
    println!("     u16 : {} 字节", size_of::<u16>());
    println!("     u32 : {} 字节", size_of::<u32>());
    println!("     u64 : {} 字节", size_of::<u64>());

    // 结构体大小
    println!("\n   结构体大小:");
    println!("     OneByte          : {} 字节", size_of::<OneByte>());
    println!("     TwoByte          : {} 字节", size_of::<TwoByte>());
    println!("     ThreeByte        : {} 字节", size_of::<ThreeByte>());
    println!("     ThreeBytePacked  : {} 字节", size_of::<ThreeBytePacked>());
    println!("     FourByte         : {} 字节", size_of::<FourByte>());
    println!("     MixedType        : {} 字节", size_of::<MixedType>());

    // 对齐要求
    println!("\n   对齐要求:");
    println!("     OneByte          : {} 字节", align_of::<OneByte>());
    println!("     TwoByte          : {} 字节", align_of::<TwoByte>());
    println!("     ThreeByte        : {} 字节", align_of::<ThreeByte>());
    println!("     ThreeBytePacked  : {} 字节", align_of::<ThreeBytePacked>());
    println!("     FourByte         : {} 字节", align_of::<FourByte>());
    println!("     MixedType        : {} 字节", align_of::<MixedType>());

    // 分析填充字节
    let normal_size = size_of::<ThreeByte>();
    let packed_size = size_of::<ThreeBytePacked>();
    let padding = normal_size - packed_size;

    println!("\n   📊 填充字节分析:");
    println!("     ThreeByte 正常布局: {} 字节", normal_size);
    println!("     ThreeBytePacked   : {} 字节", packed_size);
    println!("     填充字节          : {} 字节", padding);

    if padding > 0 {
        println!("     💡 使用 packed 可以节省 {} 字节", padding);
        println!("     ⚠️  但可能影响性能和可移植性");
    }

    println!();
}

/// 演示嵌套结构体的内存布局
fn demonstrate_nested_layout() {
    println!("🏗️  2. 嵌套结构体内存布局:");
    println!("   展示嵌套类型如何影响内存布局");

    // 单独的内部结构体
    let inner_size = size_of::<Inner>();
    let inner_align = align_of::<Inner>();
    println!("   Inner 结构体:");
    println!("     大小: {} 字节", inner_size);
    println!("     对齐: {} 字节", inner_align);

    // 嵌套的外部结构体
    let outer_size = size_of::<Outer>();
    let outer_align = align_of::<Outer>();
    println!("\n   Outer 结构体:");
    println!("     大小: {} 字节", outer_size);
    println!("     对齐: {} 字节", outer_align);

    // 分析嵌套影响
    let expected_size = inner_size + size_of::<u32>();
    let padding = outer_size - expected_size;

    println!("\n   📊 嵌套分析:");
    println!("     Inner 大小     : {} 字节", inner_size);
    println!("     u32 大小        : {} 字节", size_of::<u32>());
    println!("     理论总大小    : {} 字节", expected_size);
    println!("     实际总大小    : {} 字节", outer_size);
    println!("     填充字节      : {} 字节", padding);

    // 创建实例并展示内存布局
    let inner = Inner { a: 0x1234, b: 0x56 };
    let outer = Outer { inner, value: 0x89ABCDEF };

    println!("\n   🎯 实例值:");
    println!("     Inner {{ a: 0x{:04X}, b: 0x{:02X} }}", inner.a, inner.b);
    println!("     Outer {{ inner: {{...}}, value: 0x{:08X} }}", outer.value);

    println!();
}

/// 演示对齐控制的效果
fn demonstrate_alignment_control() {
    println!("📏 3. 对齐控制演示:");
    println!("   展示不同对齐属性的效果");

    // 普通结构体
    let normal_size = size_of::<FourByte>();
    let normal_align = align_of::<FourByte>();
    println!("   普通 FourByte:");
    println!("     大小: {} 字节", normal_size);
    println!("     对齐: {} 字节", normal_align);

    // 强制对齐的结构体
    let aligned_size = size_of::<Aligned16>();
    let aligned_align = align_of::<Aligned16>();
    println!("\n   Aligned16 (align(16)):");
    println!("     大小: {} 字节", aligned_size);
    println!("     对齐: {} 字节", aligned_align);

    // 计算额外开销
    let overhead = aligned_size - normal_size;
    println!("\n   📊 对齐分析:");
    println!("     普通对齐开销  : 0 字节");
    println!("     强制对齐开销  : {} 字节", overhead);

    if overhead > 0 {
        println!("     💡 强制对齐可以用于 SIMD 或硬件接口");
        println!("     ⚠️  但会增加内存使用量");
    }

    // 透明包装测试
    let wrapper_size = size_of::<Wrapper<u32>>();
    let wrapped_size = size_of::<u32>();
    println!("\n   透明包装测试:");
    println!("     Wrapper<u32> 大小: {} 字节", wrapper_size);
    println!("     u32 大小        : {} 字节", wrapped_size);
    println!("     布局相同        : {}", wrapper_size == wrapped_size);

    println!();
}

/// 演示联合体的内存重叠
fn demonstrate_union_layout() {
    println!("🔀 4. 联合体内存布局演示:");
    println!("   展示联合体如何共享内存");

    let union_size = size_of::<DataUnion>();
    let union_align = align_of::<DataUnion>();
    let u32_size = size_of::<u32>();
    let parts_size = size_of::<Parts>();

    println!("   联合体信息:");
    println!("     DataUnion 大小: {} 字节", union_size);
    println!("     DataUnion 对齐: {} 字节", union_align);
    println!("     u32 大小      : {} 字节", u32_size);
    println!("     Parts 大小    : {} 字节", parts_size);

    println!("\n   📊 联合体分析:");
    println!("     最大字段大小  : {} 字节", u32_size.max(parts_size));
    println!("     联合体大小    : {} 字节", union_size);
    println!("     内存共享      : 是");

    // 演示内存重叠
    let data = 0x12345678u32;
    let union_data = DataUnion { data };

    println!("\n   🎯 内存重叠演示:");
    println!("     设置 data = 0x{:08X}", data);

    // 安全地访问联合体
    unsafe {
        println!("     读取 data  = 0x{:08X}", union_data.data);
        println!("     读取 parts.low  = 0x{:04X}", union_data.parts.low);
        println!("     读取 parts.high = 0x{:04X}", union_data.parts.high);
    }

    println!("     💡 同一块内存可以用不同方式解释");
    println!("     ⚠️  访问联合体需要 unsafe 代码");

    println!();
}

/// 演示枚举的内存布局
fn demonstrate_enum_layout() {
    println!("🏷️  5. 枚举内存布局演示:");
    println!("   展示不同枚举类型的内存占用");

    // 无字段枚举
    #[repr(u8)]
    #[derive(Debug, Clone, Copy)]
    enum SmallEnum {
        A = 0,
        B = 1,
        C = 2,
    }

    let small_size = size_of::<SmallEnum>();
    let small_align = align_of::<SmallEnum>();
    println!("   小枚举 (repr(u8)):");
    println!("     大小: {} 字节", small_size);
    println!("     对齐: {} 字节", small_align);

    // 有字段枚举
    let option_size = size_of::<OptionEnum>();
    let option_align = align_of::<OptionEnum>();
    println!("\n   有字段枚举 (Option<u32>):");
    println!("     大小: {} 字节", option_size);
    println!("     对齐: {} 字节", option_align);

    // Rust 的 Option 优化
    let rust_option_size = size_of::<Option<u32>>();
    println!("\n   Rust Option<u32>:");
    println!("     大小: {} 字节", rust_option_size);
    println!("     优化: 空指针优化");

    println!("\n   📊 枚举分析:");
    println!("     Rust 对 Option 进行了特殊优化");
    println!("     None 值使用空指针表示");
    println!("     Some(value) 直接存储值");

    println!();
}

/// 演示网络包的字节序处理
fn demonstrate_network_packing() {
    println!("🌐 6. 网络包字节序演示:");
    println!("   展示网络编程中的内存布局考虑");

    let packet_size = size_of::<NetworkPacket>();
    let packet_align = align_of::<NetworkPacket>();

    println!("   NetworkPacket 信息:");
    println!("     大小: {} 字节", packet_size);
    println!("     对齐: {} 字节", packet_align);

    // 创建网络包
    let packet = NetworkPacket {
        magic: 0xDEADBEEF,
        version: 1,
        length: 42,
        checksum: 0x12345678,
    };

    // 复制 packed 字段到本地变量以避免未定义行为
    let magic = packet.magic;
    let version = packet.version;
    let length = packet.length;
    let checksum = packet.checksum;

    println!("\n   🎯 网络包示例:");
    println!("     magic    : 0x{:08X}", magic);
    println!("     version  : {}", version);
    println!("     length   : {}", length);
    println!("     checksum : 0x{:08X}", checksum);

    println!("\n   📊 网络包分析:");
    println!("     总大小    : {} 字节", packet_size);
    println!("     字段数量   : 4 个");
    println!("     packed 布局: 是（无填充）");

    // 模拟网络传输
    println!("\n   🌐 网络传输考虑:");
    println!("     网络字节序  : 大端序 (Big Endian)");
    println!("     主机字节序  : 需要转换");
    println!("     填充处理   : packed 避免填充问题");

    // 演示字节序转换
    let host_value = 0x12345678u32;
    let network_value = host_value.to_be(); // 转换为大端序
    let converted_back = u32::from_be(network_value); // 从大端序转换回来

    println!("\n   🔄 字节序转换:");
    println!("     主机字节序   : 0x{:08X}", host_value);
    println!("     网络字节序   : 0x{:08X}", network_value);
    println!("     转换回来     : 0x{:08X}", converted_back);
    println!("     转换正确     : {}", host_value == converted_back);

    println!();
}

/// 演示位字段和位操作
fn demonstrate_bit_fields() {
    println!("⚙️  7. 位字段演示:");
    println!("   展示位操作和内存打包技巧");

    // 基础位操作
    println!("   基础位操作:");
    let flags: u8 = 0b1010_1100;
    println!("     原始值: 0b{:08b}", flags);
    println!("     设置位 2: 0b{:08b}", flags | 0b0000_0100);
    println!("     清除位 3: 0b{:08b}", flags & !0b0000_1000);
    println!("     切换位 1: 0b{:08b}", flags ^ 0b0000_0010);

    // 使用 bitflags
    println!("\n   🏷️  权限标志位:");
    let read_perm = FilePermissions::READ;
    let write_perm = FilePermissions::WRITE;
    let all_perm = FilePermissions::ALL;

    println!("     READ    : {:?}", read_perm);
    println!("     WRITE   : {:?}", write_perm);
    println!("     ALL     : {:?}", all_perm);

    let user_perms = FilePermissions::READ | FilePermissions::WRITE;
    println!("\n   用户权限: {:?}", user_perms);
    println!("     包含读取: {}", user_perms.contains(FilePermissions::READ));
    println!("     包含写入: {}", user_perms.contains(FilePermissions::WRITE));
    println!("     包含执行: {}", user_perms.contains(FilePermissions::EXECUTE));

    // 位字段打包
    println!("\n   📦 位字段打包:");
    let packed_flags: u8 = 0b0000_1101;
    println!("     打包值: 0b{:08b}", packed_flags);
    println!("     解包 - 读取位 0: {}", (packed_flags & 0b0000_0001) != 0);
    println!("     解包 - 读取位 2: {}", (packed_flags & 0b0000_0100) != 0);
    println!("     解包 - 读取位 3: {}", (packed_flags & 0b0000_1000) != 0);

    // 内存节省分析
    let bool_flags_size = size_of::<[bool; 8]>();
    let bit_flags_size = size_of::<u8>();

    println!("\n   📊 内存节省分析:");
    println!("     8个 bool 数组 : {} 字节", bool_flags_size);
    println!("     1个 u8 位字段: {} 字节", bit_flags_size);
    println!("     节省内存     : {} 字节", bool_flags_size - bit_flags_size);
    println!("     压缩比例     : {:.1}x", bool_flags_size as f64 / bit_flags_size as f64);

    println!();
}

/// 演示序列化布局
fn demonstrate_serialization_layout() {
    println!("📦 8. 序列化布局演示:");
    println!("   展示序列化时的内存布局考虑");

    // 创建测试数据
    let data = SerializableData {
        id: 12345,
        name: "测试数据".to_string(),
        timestamp: 1634567890,
        data: vec![1, 2, 3, 4, 5],
    };

    // JSON 序列化
    let json_str = serde_json::to_string(&data).unwrap();
    let json_size = json_str.len();

    println!("   🎯 测试数据:");
    println!("     ID: {}", data.id);
    println!("     名称: {}", data.name);
    println!("     时间戳: {}", data.timestamp);
    println!("     数据长度: {}", data.data.len());

    println!("\n   📄 JSON 序列化:");
    println!("     序列化大小: {} 字节", json_size);
    println!("     序列化内容: {}", json_str);

    // 二进制序列化
    let bin_data = bincode::serialize(&data).unwrap();
    let bin_size = bin_data.len();

    println!("\n   🔢 二进制序列化:");
    println!("     序列化大小: {} 字节", bin_size);
    println!("     压缩比例  : {:.1}x", json_size as f64 / bin_size as f64);

    // 内存中大小对比
    let mem_size = size_of::<SerializableData>();
    println!("\n   📊 大小对比:");
    println!("     内存中大小  : {} 字节", mem_size);
    println!("     JSON 序列化 : {} 字节", json_size);
    println!("     二进制序列化: {} 字节", bin_size);
    println!("     最紧凑存储 : {}", bin_size.min(json_size).min(mem_size));

    println!("\n   💡 序列化考虑:");
    println!("     JSON: 可读性好，但体积大");
    println!("     二进制: 体积小，但可读性差");
    println!("     选择取决于具体需求");

    println!();
}

/// 演示性能影响
fn demonstrate_performance_impact() {
    println!("⚡ 9. 性能影响演示:");
    println!("   展示不同内存布局对性能的影响");

    use std::time::Instant;

    // 测试数据
    const ITERATIONS: usize = 100_000;

    // 测试正常对齐访问
    println!("   正常对齐访问测试:");
    let start = Instant::now();
    let mut sum = 0u32;

    for i in 0..ITERATIONS {
        let data = FourByte { a: i as u32 };
        sum = sum.wrapping_add(data.a);
    }

    let normal_time = start.elapsed();
    println!("     时间: {:?}", normal_time);
    println!("     结果: {}", sum);

    // 测试 packed 访问
    println!("\n   packed 访问测试:");
    let start = Instant::now();
    let mut sum = 0u16;

    for i in 0..ITERATIONS {
        let data = ThreeBytePacked { a: (i % 65536) as u16, b: (i % 256) as u8 };
        let a_value = data.a;  // 复制到本地变量以避免未定义行为
        sum = sum.wrapping_add(a_value);
    }

    let packed_time = start.elapsed();
    println!("     时间: {:?}", packed_time);
    println!("     结果: {}", sum);

    // 性能对比
    println!("\n   📊 性能对比:");
    if normal_time < packed_time {
        let slowdown = packed_time.as_nanos() as f64 / normal_time.as_nanos() as f64;
        println!("     packed 访问慢了 {:.2}x", slowdown);
        println!("     💡 正常对齐访问性能更好");
    } else {
        println!("     ⚠️  在这个平台上性能差异不明显");
    }

    // 内存使用对比
    let normal_size = size_of::<FourByte>();
    let packed_size = size_of::<ThreeBytePacked>();
    let memory_saving = normal_size - packed_size;

    println!("\n   💾 内存使用对比:");
    println!("     正常布局: {} 字节", normal_size);
    println!("     packed 布局: {} 字节", packed_size);
    println!("     内存节省: {} 字节", memory_saving);
    println!("     节省比例: {:.1}%", memory_saving as f64 / normal_size as f64 * 100.0);

    println!("\n   💡 性能建议:");
    println!("     优先使用正常对齐，除非有特殊需求");
    println!("     packed 可以节省内存，但可能影响性能");
    println!("     选择取决于具体应用场景");

    println!();
}

// ==================== 主函数 ====================

fn main() {
    println!("=== Rust 内存打包和对齐深入学习示例 ===\n");

    println!("本示例将演示内存打包、对齐和布局的概念，");
    println!("这是理解 Rust 内存管理和性能优化的关键概念。\n");

    println!("🚀 开始学习之旅...\n");

    // 1. 基础内存布局
    demonstrate_basic_layout();

    // 2. 嵌套结构体布局
    demonstrate_nested_layout();

    // 3. 对齐控制
    demonstrate_alignment_control();

    // 4. 联合体布局
    demonstrate_union_layout();

    // 5. 枚举布局
    demonstrate_enum_layout();

    // 6. 网络包打包
    demonstrate_network_packing();

    // 7. 位字段
    demonstrate_bit_fields();

    // 8. 序列化布局
    demonstrate_serialization_layout();

    // 9. 性能影响
    demonstrate_performance_impact();

    println!("=== 内存打包和对齐学习总结 ===");
    println!("🎯 核心概念回顾:");
    println!("  • 内存对齐提高访问性能但可能浪费空间");
    println!("  • packed 属性节省空间但可能影响性能");
    println!("  • 联合体提供内存重叠的高级功能");
    println!("  • 网络编程需要考虑字节序问题");
    println!();
    println!("💡 最佳实践:");
    println!("  • 优先使用 Rust 的默认对齐");
    println!("  • 仅在必要时使用 packed");
    println!("  • 网络协议要明确字节序");
    println!("  • 使用位字段节省内存");
    println!();
    println!("🔧 实际应用:");
    println!("  • 网络协议解析和构造");
    println!("  • 文件格式处理");
    println!("  • 嵌入式系统编程");
    println!("  • 高性能数据结构设计");
    println!();
    println!("✅ 学习完成！您已经掌握了 Rust 内存布局的核心概念。");
}