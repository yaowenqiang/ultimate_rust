// 导入标准库模块
use std::collections::{HashMap, hash_map}; // 哈希映射集合
use std::fmt::Debug; // 调试格式化 trait
use std::hash::Hash; // 哈希计算 trait

// 导入高级泛型示例
mod advanced_generics;
use advanced_generics::demonstrate_advanced_generics;

/*
 * Rust 泛型 (Generics) 全面学习示例
 *
 * 本文件展示了 Rust 泛型系统的强大功能，从基础概念到高级应用，
 * 包含详细的中文注释、实用示例和最佳实践指导。
 *
 * 📚 官方文档链接：
 *
 * 🔰 基础概念
 * 1. Rust Book - 泛型数据类型:
 *    https://doc.rust-lang.org/book/ch10-01-syntax.html
 *
 * 2. Rust by Example - 泛型基础:
 *    https://doc.rust-lang.org/rust-by-example/generics.html
 *
 * ⚙️ 高级概念
 * 3. 高级 Trait 和生命周期:
 *    https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
 *
 * 4. 泛型约束和 where 子句:
 *    https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods
 *
 * 🔄 类型转换
 * 5. From trait 完整文档:
 *    https://doc.rust-lang.org/std/convert/trait.From.html
 *
 * 6. Into trait 文档:
 *    https://doc.rust-lang.org/std/convert/trait.Into.html
 *
 * 🔄 迭代器深入
 * 7. Iterator trait 完整文档:
 *    https://doc.rust-lang.org/std/iter/trait.Iterator.html
 *
 * 8. 迭代器模式详解:
 *    https://doc.rust-lang.org/book/ch13-02-iterators-and-closures.html
 *
 * 9. 迭代器适配器方法:
 *    https://doc.rust-lang.org/std/iter/index.html#adapters
 *
 * 🏗️ 生命周期和内存安全
 * 10. 生命周期标注详解:
 *     https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
 *
 * 11. 生命周期子类型化:
 *     https://doc.rust-lang.org/nomicon/lifetime-subtyping.html
 *
 * 🚀 性能和优化
 * 12. 单态化和零成本抽象:
 *     https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics
 *
 * 13. 编译时优化指南:
 *     https://doc.rust-lang.org/book/ch13-04-performance.html
 *
 * 🎯 实用工具和标准库
 * 14. Debug trait:
 *     https://doc.rust-lang.org/std/fmt/trait.Debug.html
 *
 * 15. Display trait:
 *     https://doc.rust-lang.org/std/fmt/trait.Display.html
 *
 * 16. PartialEq trait:
 *     https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
 *
 * 17. Hash trait:
 *     https://doc.rust-lang.org/std/hash/trait.Hash.html
 *
 * 18. HashMap 集合:
 *     https://doc.rust-lang.org/std/collections/struct.HashMap.html
 *
 * 📊 数学和常量泛型
 * 19. 常量泛型 RFC:
 *     https://rust-lang.github.io/rfcs/2000-const-generics.html
 *
 * 20. 数学和科学计算:
 *     https://doc.rust-lang.org/std/f32/index.html
 *
 * 🎯 核心学习要点：
 *
 * 🔹 类型安全与性能
 * - 泛型在编译时进行单态化，零成本抽象
 * - 类型检查在编译时完成，运行时性能等同于手写代码
 *
 * 🔹 代码复用
 * - 一套泛型代码可以适用于多种类型
 * - 减少代码重复，提高维护性
 *
 * 🔹 灵活性
 * - 支持多个泛型参数和复杂约束
 * - where 子句使复杂约束更清晰易读
 *
 * 🔹 现代 Rust 语法
 * - impl Trait 简化函数签名
 * - 泛型关联类型 (GATs) 提供更强表达能力
 *
 * 🔹 高级特性
 * - 生命周期参数确保内存安全
 * - 自定义迭代器展示泛型的高级用法
 * - 类型转换系统提供类型安全的转换
 *
 * 🏗️ 架构设计模式：
 * - 泛型结构体实现类型安全的数据容器
 * - trait bounds 提供灵活的约束系统
 * - 迭代器模式支持函数式编程
 *
 * 💡 最佳实践：
 * - 优先使用具体的类型名称 (如 K 表示键，V 表示值)
 * - 使用 where 子句而不是内联约束来提高可读性
 * - 合理使用 impl Trait 来简化 API 设计
 * - 理解单态化的性能影响，避免代码膨胀
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

// ==================== 自定义迭代器实现 ====================

/// 为 GroupedHashMap 实现自定义迭代器
/// 这个迭代器会遍历哈希桶中的所有键值对，包括重复的键
impl<K, V> GroupedHashMap<K, V> {
    /// 创建一个遍历所有键值对的迭代器
    ///
    /// # 返回值
    /// 返回 HashMapBucketIter，它会逐一返回每个键对应的每个值
    ///
    /// # 示例
    /// ```
    /// let mut bucket = GroupedHashMap::create_new();
    /// bucket.insert_value("key", 1);
    /// bucket.insert_value("key", 2);
    /// for (key, value) in bucket.iter() {
    ///     println!("{}: {}", key, value);
    /// }
    /// ```
    fn create_bucket_iterator(&self) -> HashMapBucketIter<K, V> {
        let mut key_iterator = self.internal_map.iter();
        let first_map_entry = key_iterator.next();

        HashMapBucketIter {
            key_iterator,                       // 用于遍历所有键的迭代器
            current_map_entry: first_map_entry, // 当前正在处理的键值对
            current_vec_index: 0,               // 当前向量中的索引位置
        }
    }
}

/// 自定义迭代器结构体
///
/// 这是一个展示泛型与生命周期结合的经典示例！
/// 自定义迭代器是 Rust 中一个强大的概念，它允许你创建自己的数据遍历逻辑。
///
/// # 类型参数详解
/// * `'a` - 生命周期参数，确保迭代器不会比被迭代的数据活得更久
///         这是 Rust 借用检查系统的关键部分，防止悬垂指针
/// * `K` - 键类型，通常需要实现 Eq + Hash + Debug
/// * `V` - 值类型，可以是任意类型
///
/// # 字段详解
/// * `key_iterator` - 哈希映射的键迭代器，用于遍历所有键
/// * `current_map_entry` - 当前正在处理的键值对引用，包含键和值向量
/// * `current_vec_index` - 当前值向量中的索引位置，用于追踪处理到哪个值
///
/// # 迭代器工作原理
/// 这个迭代器实现了"扁平化"遍历：将嵌套的 HashMap<K, Vec<V>> 结构
/// 展开为一系列的 (K, V) 键值对，每个键值对都是原始数据中的一个元素。
///
/// # 文档链接
/// - [迭代器模式详解](https://doc.rust-lang.org/book/ch13-02-iterators-and-closures.html)
/// - [Iterator trait 文档](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
/// - [生命周期标注](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
struct HashMapBucketIter<'a, K, V> {
    key_iterator: hash_map::Iter<'a, K, Vec<V>>, // 用于遍历 HashMap 键的迭代器
    current_map_entry: Option<(&'a K, &'a Vec<V>)>, // 当前处理的键值对：键和对应的值向量引用
    current_vec_index: usize,                    // 在当前值向量中的位置索引
}

/// 为自定义迭代器实现 Iterator trait
///
/// 这是泛型与生命周期结合的绝佳示例！
/// 实现 Iterator trait 是创建自定义迭代器的核心步骤。
///
/// # 迭代器的核心概念
/// 迭代器模式允许你遍历集合中的元素，而无需暴露集合的内部结构。
/// Rust 的迭代器是惰性的，只有在调用 next() 方法时才会计算下一个值。
///
/// # 关联类型
/// * `Item` - 迭代器产生的元素类型，这里是键值对的元组引用
///           使用引用 (&'a K, &'a V) 避免所有权转移，提高性能
///
/// # 文档链接
/// - [Iterator trait 完整文档](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
/// - [迭代器适配器方法](https://doc.rust-lang.org/std/iter/index.html#adapters)
/// - [迭代器消费器方法](https://doc.rust-lang.org/std/iter/index.html#consumers)
impl<'a, K, V> Iterator for HashMapBucketIter<'a, K, V> {
    /// 迭代器产生的元素类型
    ///
    /// 使用引用类型 (&'a K, &'a V) 的优势：
    /// 1. **避免所有权转移** - 原始数据仍然可以继续使用
    /// 2. **零成本抽象** - 编译时优化，运行时没有额外开销
    /// 3. **内存安全** - 生命周期参数确保引用始终有效
    type Item = (&'a K, &'a V);

    /// 获取下一个元素 - 迭代器的核心方法
    ///
    /// 这个方法实现了复杂的"扁平化"迭代逻辑：
    ///
    /// # 算法步骤详解
    /// 1. **检查当前键值对** - 是否还有未处理的值
    /// 2. **返回当前值** - 如果有，返回下一个值并递增索引
    /// 3. **移动到下一个键** - 如果当前键的所有值都处理完了
    /// 4. **递归处理** - 重复上述步骤直到所有键值对都被处理完
    ///
    /// # 状态管理
    /// 迭代器维护了两个关键状态：
    /// - `current_map_entry`: 当前正在处理的键值对
    /// - `current_vec_index`: 在当前值向量中的位置
    ///
    /// # 返回值语义
    /// * `Some((key, value))` - 成功获取下一个键值对
    /// * `None` - 迭代结束，所有元素都已处理完毕
    ///
    /// # 性能特点
    /// - **O(1) 平均时间复杂度** - 每次调用都是常数时间
    /// - **惰性求值** - 只在需要时才计算下一个值
    /// - **内存高效** - 不需要额外的存储空间
    ///
    /// # 文档链接
    /// - [next() 方法文档](https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next)
    /// - [迭代器设计模式](https://rust-unofficial.github.io/patterns/patterns/behavioural/iterator.html)
    fn next(&mut self) -> Option<Self::Item> {
        // 第一阶段：处理当前键值对中的所有值
        // 使用 if let 进行模式匹配，安全地解构 Option 类型
        if let Some((current_key, current_values)) = &mut self.current_map_entry {
            // 检查当前键值对的值向量中是否还有未处理的值
            if self.current_vec_index < current_values.len() {
                // 获取当前索引位置的值，并递增索引以准备下一次调用
                let current_value = &current_values[self.current_vec_index];
                self.current_vec_index += 1; // 重要：更新状态！
                return Some((current_key, current_value));
            } else {
                // 第二阶段：当前键的所有值都已处理，需要移动到下一个键

                // 从键迭代器获取下一个键值对
                self.current_map_entry = self.key_iterator.next();
                self.current_vec_index = 0; // 重置索引，准备处理新键的值向量

                // 第三阶段：处理新获取的键值对（如果存在）
                // 注意：这里需要一个嵌套检查，因为我们刚刚移动到了新键
                if let Some((next_key, next_values)) = &mut self.current_map_entry {
                    // 确保新键的值向量不为空
                    if self.current_vec_index < next_values.len() {
                        let next_value = &next_values[self.current_vec_index];
                        self.current_vec_index += 1; // 别忘了递增索引！
                        return Some((next_key, next_value));
                    }
                }
            }
        }

        // 第四阶段：所有元素都已处理完毕
        // 当 current_map_entry 为 None 时，表示我们已经遍历了所有键
        None
    }
}

// 为 GroupedHashMap 实现泛型方法
// 使用 where 子句来明确泛型约束，使代码更易读
impl<K, V> GroupedHashMap<K, V>
where
    K: Eq + Hash + Debug, // 键类型必须支持相等比较、哈希计算和调试显示
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
    T: ToString + Debug, // 第一个类型必须能转换为字符串且可调试
    U: ToString + Debug, // 第二个类型必须能转换为字符串且可调试
{
    println!(
        "第一个值: {} (调试: {:?})",
        first_value.to_string(),
        first_value
    );
    println!(
        "第二个值: {} (调试: {:?})",
        second_value.to_string(),
        second_value
    );
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

    // 基础数值比较 - 整数
    println!("\n🔢 整数比较:");
    compare_and_display(10, 20);
    compare_and_display(100, 50);
    compare_and_display(42, 42); // 相等情况

    // 浮点数比较 - 包括精度问题演示
    println!("\n🔢 浮点数比较:");
    compare_and_display(3.14, 2.71);
    compare_and_display(-1.5, -3.8);
    compare_and_display(0.0, -0.0); // 正负零比较

    // 浮点数精度比较演示 - 重要概念！
    println!("\n🔍 浮点数精度演示 (重要概念):");
    let float_a: f64 = 0.1 + 0.2; // 这实际上不等于 0.3！
    let float_b: f64 = 0.3;
    println!("0.1 + 0.2 = {:.15}", float_a);
    println!("0.3 = {:.15}", float_b);
    println!(
        "直接比较结果: {} == {}? {}",
        float_a,
        float_b,
        float_a == float_b
    );

    // 使用近似比较解决精度问题
    let epsilon: f64 = 1e-10;
    let approx_equal = (float_a - float_b).abs() < epsilon;
    println!(
        "近似比较 (ε = 1e-10): {} ≈ {}? {}",
        float_a, float_b, approx_equal
    );

    // 演示浮点数比较的陷阱
    compare_and_display(float_a, float_b);

    // 字符串比较（按字典序）
    println!("\n📝 字符串比较 (字典序):");
    compare_and_display("hello", "world");
    compare_and_display("rust", "rust");
    compare_and_display("Apple", "apple"); // 大小写敏感
    compare_and_display("", "空字符串"); // 空字符串比较

    // 中文字符串比较
    println!("\n📝 中文字符串比较:");
    compare_and_display("北京", "上海");
    compare_and_display("你好", "世界");
    compare_and_display("编程", "编程");

    // 字符比较
    println!("\n🔤 字符比较:");
    compare_and_display('A', 'Z');
    compare_and_display('中', '文');
    compare_and_display('a', 'A'); // 大小写比较
    compare_and_display('1', '9'); // 数字字符

    // Unicode 字符比较演示
    println!("\n🌍 Unicode 字符比较演示:");
    let unicode_chars = ['α', 'β', 'γ', 'δ', 'ε'];
    for (i, &ch1) in unicode_chars.iter().enumerate() {
        for &ch2 in unicode_chars[i + 1..].iter() {
            compare_and_display(ch1, ch2);
        }
    }

    // 复杂数值比较演示
    println!("\n🔬 复杂数值比较:");

    // 科学计数法
    compare_and_display(1.23e4, 1.24e4);

    // 负数比较
    compare_and_display(-100, -50);
    compare_and_display(-3.14, -2.71);

    // 零附近的比较
    compare_and_display(1e-10, 1e-11);
    compare_and_display(-1e-10, 1e-10);

    // 类型转换比较演示
    println!("\n🔄 类型转换比较演示:");

    // 整数与浮点数
    let int_val: i32 = 42;
    let float_val: f64 = 42.0;
    // 注意：这里不能直接比较不同类型，但可以转换后比较
    println!("整数 {} 和浮点数 {} 不能直接比较", int_val, float_val);

    // 使用泛型函数演示相同值的比较
    compare_and_display(42.0, 42.0); // 浮点数
    compare_and_display(42, 42); // 整数

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

    // 更多角度转换示例 - 展示常用角度的精确转换
    println!("\n🔄 更多角度转换示例 (常用角度对照表):");
    let angles_degrees = [0.0, 30.0, 45.0, 60.0, 90.0, 180.0, 270.0, 360.0];

    println!("┌─────────┬─────────────────┬─────────────────┐");
    println!("│  度数   │     弧度值      │   正弦值 (sin)   │");
    println!("├─────────┼─────────────────┼─────────────────┤");

    for deg in angles_degrees.iter() {
        let angle_deg = AngleDegrees(*deg);
        let angle_rad: AngleRadians = angle_deg.into();
        let sine_value = angle_rad.0.sin();

        println!(
            "│ {:>6.1}° │ {:>13.6} │ {:>13.6} │",
            deg, angle_rad.0, sine_value
        );
    }
    println!("└─────────┴─────────────────┴─────────────────┘");

    // 反向转换演示 - 从弧度转换回度数
    println!("\n🔙 反向转换示例 (弧度 → 度数):");
    let common_radians = [
        (0.0, "0"),
        (std::f32::consts::PI / 6.0, "π/6"),
        (std::f32::consts::PI / 4.0, "π/4"),
        (std::f32::consts::PI / 3.0, "π/3"),
        (std::f32::consts::PI / 2.0, "π/2"),
        (std::f32::consts::PI, "π"),
        (2.0 * std::f32::consts::PI, "2π"),
    ];

    for (rad_val, rad_name) in common_radians.iter() {
        let angle_rad = AngleRadians(*rad_val);
        let angle_deg: AngleDegrees = angle_rad.into();
        println!("{} 弧度 = {:>6.1}°", rad_name, angle_deg.0);
    }

    // 类型转换的实际应用演示
    println!("\n🎯 类型转换的实际应用:");

    // 1. 三角函数计算
    let right_angle = AngleDegrees(90.0);
    let angle_rad: AngleRadians = right_angle.into();
    println!("直角 (90°) 的正弦值: {:.6}", angle_rad.0.sin());
    println!("直角 (90°) 的余弦值: {:.6}", angle_rad.0.cos());

    // 2. 角度运算演示
    let angle1 = AngleDegrees(45.0);
    let angle2 = AngleDegrees(30.0);
    let rad1: AngleRadians = angle1.into();
    let rad2: AngleRadians = angle2.into();

    // 注意：这里我们转换为弧度进行数学运算，因为数学函数通常使用弧度
    let sum_rad = AngleRadians(rad1.0 + rad2.0);
    let sum_deg: AngleDegrees = sum_rad.into();
    println!("45° + 30° = {:.1}°", sum_deg.0);

    // 3. 角度比较演示
    let acute_angle = AngleDegrees(60.0);
    let obtuse_angle = AngleDegrees(120.0);

    // 转换为弧度进行比较
    let acute_rad: AngleRadians = acute_angle.into();
    let obtuse_rad: AngleRadians = obtuse_angle.into();

    if acute_rad.0 < obtuse_rad.0 {
        println!("60° < 120° (锐角小于钝角)");
    }

    // 4. 类型转换链演示
    println!("\n🔗 类型转换链演示:");
    println!("从度数开始 → 转换为弧度 → 计算正弦值 → 转换回度数");

    let original_deg = AngleDegrees(30.0);
    println!("原始角度: {}°", original_deg.0);

    let to_rad: AngleRadians = original_deg.into();
    println!("转换为弧度: {:.6} rad", to_rad.0);

    let sine_val = to_rad.0.sin();
    println!("正弦值: {:.6}", sine_val);

    // 反正弦函数得到弧度，然后转换为度数
    let asin_rad = AngleRadians(sine_val.asin());
    let back_to_deg: AngleDegrees = asin_rad.into();
    println!("通过反正弦转回度数: {:.1}°", back_to_deg.0);
    println!(
        "验证: 原始角度 {:.1}° ≈ 转换后角度 {:.1}° ✓",
        original_deg.0, back_to_deg.0
    );

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

    // 使用自定义迭代器演示
    println!("使用自定义迭代器遍历字符串-整数哈希桶:");
    for (key, value) in string_int_bucket.create_bucket_iterator() {
        println!("键: {}, 值: {}", key, value);
    }

    // 创建不同类型的哈希桶
    let mut int_float_bucket = GroupedHashMap::create_new();
    int_float_bucket.insert_value(1, 3.14);
    int_float_bucket.insert_value(1, 2.71);
    int_float_bucket.insert_value(2, 1.618);

    // 使用自定义迭代器遍历所有键值对
    println!("使用自定义迭代器遍历整数-浮点数哈希桶:");
    for (key, value) in int_float_bucket.create_bucket_iterator() {
        println!("键: {:?}, 值: {:?}", key, value);
    }

    println!("整数-浮点数哈希桶: {:?}", int_float_bucket);

    // ==================== 自定义迭代器演示 ====================
    println!("\n6. 自定义迭代器演示:");

    // 创建一个更复杂的哈希桶来展示迭代器
    let mut complex_bucket = GroupedHashMap::create_new();
    complex_bucket.insert_value("编程语言".to_string(), "Rust");
    complex_bucket.insert_value("编程语言".to_string(), "Python");
    complex_bucket.insert_value("编程语言".to_string(), "JavaScript");
    complex_bucket.insert_value("数据库".to_string(), "PostgreSQL");
    complex_bucket.insert_value("数据库".to_string(), "MongoDB");
    complex_bucket.insert_value("框架".to_string(), "React");

    println!("复杂哈希桶调试信息: {:?}", complex_bucket);

    // 使用自定义迭代器遍历 - 展示泛型迭代器的威力
    println!("\n🔄 使用自定义迭代器遍历所有键值对:");
    let mut item_count = 0;
    for (key, value) in complex_bucket.create_bucket_iterator() {
        item_count += 1;
        println!("  项目 {}: {} = {}", item_count, key, value);
    }
    println!("✅ 总共迭代了 {} 个项目", item_count);

    // 迭代器的链式操作示例 - 展示迭代器的威力！
    println!("\n📊 迭代器的链式操作示例 (强大的函数式编程):");

    // 1. 基础过滤操作
    println!("\n🔍 1. 过滤操作演示:");
    let programming_count = complex_bucket
        .create_bucket_iterator()
        .filter(|(key, _)| *key == "编程语言") // 只选择编程语言相关的项目
        .count(); // 统计数量
    println!("编程语言相关的项目数量: {}", programming_count);

    // 2. 映射操作 - 提取所有值
    println!("\n🗂️ 2. 映射操作演示:");
    let all_values: Vec<&str> = complex_bucket
        .create_bucket_iterator()
        .map(|(_, value)| *value) // 只提取值，丢弃键
        .collect(); // 收集到向量中
    println!("所有技术栈: {:?}", all_values);

    // 3. 复杂的链式操作组合
    println!("\n⚙️ 3. 复杂链式操作演示:");

    // 按键分组并统计每个类别的项目数量
    let mut category_counts: HashMap<&str, usize> = HashMap::new();
    for (key, _) in complex_bucket.create_bucket_iterator() {
        *category_counts.entry(key).or_insert(0) += 1;
    }
    println!("各类别项目统计: {:?}", category_counts);

    // 4. 高级链式操作演示
    println!("\n🚀 4. 高级链式操作演示:");

    // 筛选、映射、排序、收集的完整流程
    let filtered_and_sorted: Vec<(&str, &str)> = complex_bucket
        .create_bucket_iterator()
        .filter(|(key, value)| {
            // 过滤条件：只保留键长度 >= 2 且值长度 >= 4 的项目
            key.len() >= 2 && value.len() >= 4
        })
        .map(|(key, value)| (key.as_str(), *value)) // 转换为 &str 类型
        .collect(); // 收集到向量

    println!("过滤后的技术栈 (键长度≥2且值长度≥4):");
    for (category, tech) in &filtered_and_sorted {
        println!("  - {}: {}", category, tech);
    }

    // 5. 条件查找操作
    println!("\n🎯 5. 条件查找操作:");

    // 查找第一个包含 "SQL" 的技术
    let sql_tech = complex_bucket
        .create_bucket_iterator()
        .find(|(_, value)| value.contains("SQL")); // 查找包含 SQL 的技术

    if let Some((category, tech)) = sql_tech {
        println!("找到包含 'SQL' 的技术: {} - {}", category, tech);
    } else {
        println!("没有找到包含 'SQL' 的技术");
    }

    // 查找所有包含 "数据" 的技术
    let data_related: Vec<(&str, &str)> = complex_bucket
        .create_bucket_iterator()
        .filter(|(key, _)| key.contains("数据") || key.contains("库"))
        .map(|(key, value)| (key.as_str(), *value))
        .collect();

    println!("数据相关技术:");
    for (category, tech) in data_related {
        println!("  - {}: {}", category, tech);
    }

    // 6. 统计和聚合操作
    println!("\n📈 6. 统计和聚合操作:");

    // 计算每个类别的项目数量
    let mut stats: HashMap<&str, usize> = HashMap::new();
    let total_items = complex_bucket
        .create_bucket_iterator()
        .map(|(key, value)| {
            // 更新统计
            *stats.entry(key).or_insert(0) += 1;
            (key, value) // 返回键值对
        })
        .count(); // 统计总数

    println!("项目总数: {}", total_items);
    println!("详细统计: {:?}", stats);

    // 7. 迭代器的性能优势演示
    println!("\n⚡ 7. 迭代器的性能优势:");

    // 惰性求值演示 - 创建迭代器但不立即执行
    let lazy_iterator = complex_bucket
        .create_bucket_iterator()
        .filter(|(key, _)| key.len() > 2) // 只选择键长度大于2的
        .map(|(key, value)| (key.to_uppercase(), value.to_uppercase())); // 转换为大写

    println!("惰性迭代器已创建，但尚未执行任何操作");

    // 现在执行迭代
    println!("执行惰性迭代器:");
    for (uppercase_key, uppercase_value) in lazy_iterator {
        println!("  {}: {}", uppercase_key, uppercase_value);
    }

    // 8. 迭代器与集合操作的对比
    println!("\n🔄 8. 迭代器 vs 传统循环:");

    // 传统方式 (for 循环)
    let mut traditional_results = Vec::new();
    for (key, value) in complex_bucket.create_bucket_iterator() {
        if value.starts_with("P") {
            traditional_results.push(format!("{}: {}", key, value));
        }
    }
    println!("传统循环结果 (以 P 开头的技术): {:?}", traditional_results);

    // 迭代器方式
    let iterator_results: Vec<String> = complex_bucket
        .create_bucket_iterator()
        .filter(|(_, value)| value.starts_with("P"))
        .map(|(key, value)| format!("{}: {}", key, value))
        .collect();
    println!("迭代器结果 (以 P 开头的技术): {:?}", iterator_results);

    println!("\n✨ 迭代器链式操作的优势:");
    println!("- 代码更简洁、可读性更强");
    println!("- 支持惰性求值，性能更优");
    println!("- 易于组合复杂的操作");
    println!("- 类型安全，编译时优化");

    // ==================== 高级泛型概念演示 ====================
    println!("\n7. 更多泛型特性演示:");

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
