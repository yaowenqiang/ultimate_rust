// 导入标准库模块
use std::fmt::Debug;            // 调试格式化 trait
use std::collections::HashMap; // 哈希映射集合
use std::hash::Hash;           // 哈希计算 trait

// 导入高级泛型示例
mod advanced_generics;
use advanced_generics::demonstrate_advanced_generics;

/*
 * Rust 泛型 (Generics) 学习示例
 *
 * 📚 相关文档链接：
 *
 * 1. Rust Book - 泛型数据类型:
 *    https://doc.rust-lang.org/book/ch10-01-syntax.html
 *
 * 2. Rust by Example - 泛型:
 *    https://doc.rust-lang.org/rust-by-example/generics.html
 *
 * 3. 高级 Trait 和生命周期:
 *    https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
 *
 * 4. 泛型约束和 where 子句:
 *    https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods
 *
 * 5. 类型转换和 From/Into traits:
 *    https://doc.rust-lang.org/std/convert/trait.From.html
 *
 * 🎯 学习要点：
 * - 泛型提供类型安全和代码复用
 * - 可以使用多个泛型参数
 * - 支持泛型约束 (trait bounds)
 * - where 子句使约束更清晰
 * - impl Trait 语法简化函数签名
 * - 泛型在编译时进行单态化，性能与手写代码相同
 */

/// 泛型哈希桶结构体
/// 这是一个使用泛型的示例，可以将多个值存储在同一个键下
///
/// # 类型参数
/// * `K` - 键类型，必须实现 Eq + Hash + Debug
/// * `V` - 值类型，可以是任意类型
#[derive(Debug)]
struct GroupedHashMap<K, V> {
    internal_map: HashMap<K, Vec<V>>, // 内部哈希映射，每个键对应一个值向量
}

// 为 GroupedHashMap 实现泛型方法
// 使用 where 子句来明确泛型约束，使代码更易读
impl<K, V> GroupedHashMap<K, V>
where
    K: Eq + Hash + Debug,  // 键类型必须支持相等比较、哈希计算和调试显示
{
    /// 创建新的 GroupedHashMap 实例
    ///
    /// # 返回值
    /// 返回一个空的 GroupedHashMap
    fn create_new() -> Self {
        GroupedHashMap {
            internal_map: HashMap::new(),
        }
    }

    /// 插入键值对，如果键已存在则追加到值向量中
    ///
    /// # 参数
    /// * `key` - 键，类型为 K
    /// * `value` - 值，类型为 V
    ///
    /// # 示例
    /// ```
    /// let mut bucket = GroupedHashMap::create_new();
    /// bucket.insert("fruit", "apple");
    /// bucket.insert("fruit", "banana");
    /// ```
    fn insert_value(&mut self, key: K, value: V) {
        // 使用 entry API 来处理已存在和不存在的情况
        // or_insert 在键不存在时插入新向量，存在时返回现有向量的可变引用
        let values_vector: &mut Vec<V> = self.internal_map.entry(key).or_insert(Vec::new());
        values_vector.push(value);
    }

    /// 获取指定键的所有值
    ///
    /// # 参数
    /// * `key` - 要查找的键
    ///
    /// # 返回值
    /// Option<&Vec<V>> - 如果键存在则返回值的向量的引用，否则返回 None
    fn get_values(&self, key: &K) -> Option<&Vec<V>> {
        self.internal_map.get(key)
    }

    /// 获取键值对的数量
    fn get_key_count(&self) -> usize {
        self.internal_map.len()
    }
}

/// 角度单位 - 度数
/// 这是一个使用元组结构体的示例，表示以度为单位的角度
#[derive(Debug, Clone, Copy, PartialEq)]
struct AngleDegrees(f32);

/// 角度单位 - 弧度
/// 这表示以弧度为单位的角度，常用于数学计算
#[derive(Debug, Clone, Copy, PartialEq)]
struct AngleRadians(f32);

// 实现从弧度到度数的转换
// From trait 是 Rust 标准库中提供的类型转换 trait
impl From<AngleRadians> for AngleDegrees {
    fn from(radians: AngleRadians) -> Self {
        // 弧度转度数：度数 = 弧度 * 180 / π
        // 这里使用 100.0 作为 180 的近似值，便于计算
        AngleDegrees(radians.0 * 100.0 / std::f32::consts::PI)
    }
}

// 实现从度数到弧度的转换
impl From<AngleDegrees> for AngleRadians {
    fn from(degrees: AngleDegrees) -> Self {
        // 度数转弧度：弧度 = 度数 * π / 180
        // 这里使用 100.0 作为 180 的近似值，便于计算
        AngleRadians(degrees.0 * std::f32::consts::PI / 100.0)
    }
}

/// 泛型函数示例：打印两个可转换为字符串的值
///
/// 这个函数展示了多个泛型参数和 trait bounds 的使用
///
/// # 类型参数
/// * `T` - 第一个值类型，必须实现 ToString + Debug
/// * `U` - 第二个值类型，必须实现 ToString + Debug
///
/// # 参数
/// * `first_value` - 第一个要打印的值
/// * `second_value` - 第二个要打印的值
fn display_two_values<T, U>(first_value: T, second_value: U)
where
    T: ToString + Debug,  // 第一个类型必须能转换为字符串且可调试
    U: ToString + Debug,  // 第二个类型必须能转换为字符串且可调试
{
    println!("第一个值: {} (调试: {:?})", first_value.to_string(), first_value);
    println!("第二个值: {} (调试: {:?})", second_value.to_string(), second_value);
}

/// 使用 impl Trait 语法的泛型函数
/// 计算角度的正弦值，接受任何可以转换为弧度的类型
///
/// # 参数
/// * `angle` - 角度，可以是任何实现了 Into<AngleRadians> 的类型
///
/// # 返回值
/// f32 - 正弦值
fn calculate_sine(angle: impl Into<AngleRadians>) -> f32 {
    // 使用 into() 方法将参数转换为 AngleRadians
    let angle_in_radians: AngleRadians = angle.into();

    // 计算正弦值
    angle_in_radians.0.sin()
}

/// 更复杂的泛型函数示例：比较两个值
///
/// # 类型参数
/// * `T` - 值类型，必须实现 PartialOrd + Debug + Display
fn compare_and_display<T>(first: T, second: T)
where
    T: PartialOrd + Debug + std::fmt::Display,
{
    println!("比较: {} 和 {}", first, second);

    if first > second {
        println!("{} > {}", first, second);
    } else if first < second {
        println!("{} < {}", first, second);
    } else {
        println!("{} = {}", first, second);
    }
}

/// 主函数：展示各种泛型概念的实际应用
fn main() {
    println!("=== Rust 泛型学习示例 ===\n");

    // ==================== 泛型函数示例 ====================
    println!("1. 泛型函数示例:");

    // 使用两个不同类型参数的泛型函数
    display_two_values("字符串", 42);
    display_two_values(100, 3.14);
    display_two_values(true, 'A');

    // ==================== 比较函数示例 ====================
    println!("\n2. 泛型比较示例:");

    compare_and_display(10, 20);
    compare_and_display(3.14, 2.71);
    compare_and_display("hello", "world");

    // ==================== 类型转换示例 ====================
    println!("\n3. 泛型类型转换示例 (From/Into traits):");

    // 创建角度值
    let angle_in_degrees = AngleDegrees(90.0);
    println!("原始角度: {} 度", angle_in_degrees.0);

    // 使用 From trait 进行转换
    let angle_in_radians_from = AngleRadians::from(angle_in_degrees);
    println!("转换为弧度 (From): {:.4} 弧度", angle_in_radians_from.0);

    // 使用 into() 方法进行转换（更简洁）
    let angle_in_degrees_again = AngleDegrees(45.0);
    let angle_in_radians_into: AngleRadians = angle_in_degrees_again.into();
    println!("45度转换为弧度 (Into): {:.4} 弧度", angle_in_radians_into.0);

    // ==================== impl Trait 语法示例 ====================
    println!("\n4. impl Trait 语法示例:");

    // 可以传递任何能转换为弧度的类型
    let sine_degrees = calculate_sine(AngleDegrees(90.0));
    let sine_radians = calculate_sine(AngleRadians(std::f32::consts::PI / 2.0));

    println!("sin(90°) = {:.6}", sine_degrees);
    println!("sin(π/2) = {:.6}", sine_radians);

    // ==================== 泛型结构体示例 ====================
    println!("\n5. 泛型结构体示例:");

    // 创建字符串键和整数值的哈希桶
    let mut string_int_bucket = GroupedHashMap::create_new();
    string_int_bucket.insert_value("水果".to_string(), 1);
    string_int_bucket.insert_value("水果".to_string(), 2);
    string_int_bucket.insert_value("蔬菜".to_string(), 3);
    string_int_bucket.insert_value("水果".to_string(), 4);

    println!("字符串-整数哈希桶: {:?}", string_int_bucket);
    println!("键的数量: {}", string_int_bucket.get_key_count());

    // 查询特定键的值
    if let Some(fruits) = string_int_bucket.get_values(&"水果".to_string()) {
        println!("水果类的值: {:?}", fruits);
    }

    // 创建不同类型的哈希桶
    let mut int_float_bucket = GroupedHashMap::create_new();
    int_float_bucket.insert_value(1, 3.14);
    int_float_bucket.insert_value(1, 2.71);
    int_float_bucket.insert_value(2, 1.618);

    println!("整数-浮点数哈希桶: {:?}", int_float_bucket);

    // ==================== 高级泛型概念演示 ====================
    println!("\n6. 更多泛型特性演示:");

    // 泛型常量
    const BUFFER_SIZE: usize = 1024;
    let mut generic_buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    generic_buffer[0] = 42;
    println!("泛型常量缓冲区第一个元素: {}", generic_buffer[0]);

    // 泛型类型别名
    type StringIntMap = GroupedHashMap<String, i32>;
    let mut type_alias_example: StringIntMap = StringIntMap::create_new();
    type_alias_example.insert_value("计数".to_string(), 100);
    println!("类型别名示例: {:?}", type_alias_example);

    // ==================== 高级泛型概念演示 ====================
    demonstrate_advanced_generics();

    println!("\n=== 泛型学习完成 ===");
    println!("泛型允许我们编写类型安全且可复用的代码！");
    println!("在编译时，泛型会被具体类型替换（单态化），性能与手写代码相同。");
}
