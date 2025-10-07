/*
 * 高级 Trait 概念示例
 *
 * 本文件展示更复杂的 trait 用法和概念
 */

use std::fmt::{Debug, Display};
use std::ops::{Add, Mul};

// ==================== 1. 关联类型 (Associated Types) ====================

// 定义一个迭代器 trait，使用关联类型
trait Iterator {
    type Item;  // 关联类型，每个实现必须指定 Item 的具体类型

    fn next(&mut self) -> Option<Self::Item>;
}

// 定义一个简单的数字计数器
struct NumberCounter {
    current_count: usize,
    maximum_count: usize,
}

impl NumberCounter {
    fn new(maximum_count: usize) -> NumberCounter {
        NumberCounter { current_count: 0, maximum_count }
    }
}

// 为 NumberCounter 实现 Iterator trait
impl Iterator for NumberCounter {
    type Item = usize;  // 指定关联类型的具体类型

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_count < self.maximum_count {
            let current_value = self.current_count;
            self.current_count += 1;
            Some(current_value)
        } else {
            None
        }
    }
}

// ==================== 2. 泛型参数 vs 关联类型 ====================

// 使用泛型参数的方式（较少用）
trait IteratorGeneric<T> {
    fn next(&mut self) -> Option<T>;
}

// 使用关联类型的方式（推荐）
trait IteratorAssociated {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// ==================== 3. Trait Bounds (Trait 约束) ====================

// 在函数中使用 trait bounds
fn display_and_compare_items<T: Display + PartialEq>(first_item: &T, second_item: &T) {
    println!("第一个项: {}", first_item);
    println!("第二个项: {}", second_item);

    if first_item == second_item {
        println!("两项相等！");
    } else {
        println!("两项不相等！");
    }
}

// 在结构体定义中使用 trait bounds
struct ComparablePair<T: Display + PartialOrd> {
    first_value: T,
    second_value: T,
}

impl<T: Display + PartialOrd> ComparablePair<T> {
    fn find_and_display_maximum(&self) {
        if self.first_value >= self.second_value {
            println!("最大的项是: {}", self.first_value);
        } else {
            println!("最大的项是: {}", self.second_value);
        }
    }
}

// ==================== 4. 覆盖实现 (Coherence Rules) ====================

// 定义一个简单 trait
trait Summarizable {
    fn generate_summary(&self) -> String {
        String::from("(阅读更多...)")
    }
}

// 为所有类型实现默认的 Summary（需要特殊语法）
// impl<T> Summary for T {
//     fn summarize(&self) -> String {
//         String::from("(默认摘要)")
//     }
// }

// 为特定类型实现
#[derive(Debug, Clone)]
struct NewsArticle {
    article_title: String,
    article_content: String,
}

impl Summarizable for NewsArticle {
    fn generate_summary(&self) -> String {
        // 安全的字符串截断，避免字符边界错误
        let content_summary = if self.article_content.len() > 50 {
            let mut end = 50;
            // 向前查找字符边界
            while !self.article_content.is_char_boundary(end) && end > 0 {
                end -= 1;
            }
            &self.article_content[..end]
        } else {
            &self.article_content
        };
        format!("{}: {}", self.article_title, content_summary)
    }
}

// ==================== 5. Trait 对象 vs 泛型 ====================

// 使用泛型（静态分发，编译时确定类型）
fn process_generic_item<T: Summarizable>(item: T) {
    println!("泛型处理: {}", item.generate_summary());
}

// 使用 trait 对象（动态分发，运行时确定类型）
fn process_trait_object_item(item: &dyn Summarizable) {
    println!("动态处理: {}", item.generate_summary());
}

// ==================== 6. 高级 Trait 概念 ====================

// 6.1 Supertraits (超特征)
// 一个 trait 可以依赖于另一个 trait
trait Printable: Display {
    fn print(&self) {
        println!("打印: {}", self);
    }
}

// 6.2 条件方法实现
trait AdvancedIterator: Iterator {
    // 只有当 Item 实现了 Clone 时才能使用这个方法
    fn cloned_items(&self) -> Vec<Self::Item>
    where
        Self: Clone,
        Self::Item: Clone,
    {
        // 实现会被编译器优化掉，如果条件不满足
        unimplemented!("这是一个条件方法的示例")
    }
}

// 6.3 运算符重载的完整示例
#[derive(Debug, Clone, Copy)]
struct TwoDVector {
    x_component: f64,
    y_component: f64,
}

impl TwoDVector {
    fn new(x_component: f64, y_component: f64) -> Self {
        TwoDVector { x_component, y_component }
    }

    fn calculate_magnitude(&self) -> f64 {
        (self.x_component * self.x_component + self.y_component * self.y_component).sqrt()
    }
}

// 向量加法
impl Add for TwoDVector {
    type Output = TwoDVector;

    fn add(self, other_vector: TwoDVector) -> TwoDVector {
        TwoDVector {
            x_component: self.x_component + other_vector.x_component,
            y_component: self.y_component + other_vector.y_component,
        }
    }
}

// 向量与标量乘法
impl Mul<f64> for TwoDVector {
    type Output = TwoDVector;

    fn mul(self, scalar_multiplier: f64) -> TwoDVector {
        TwoDVector {
            x_component: self.x_component * scalar_multiplier,
            y_component: self.y_component * scalar_multiplier,
        }
    }
}

impl Display for TwoDVector {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "({}, {})", self.x_component, self.y_component)
    }
}

// ==================== 7. 实用工具函数 ====================

// 演示各种 trait 概念的函数
pub fn demonstrate_advanced_traits() {
    println!("=== 高级 Trait 概念演示 ===\n");

    // 1. 关联类型示例
    println!("1. 关联类型示例:");
    let mut number_counter = NumberCounter::new(5);
    println!("数字计数器输出: ");
    while let Some(count_value) = number_counter.next() {
        print!("{} ", count_value);
    }
    println!("\n");

    // 2. Trait bounds 示例
    println!("2. Trait bounds 示例:");
    let comparable_pair = ComparablePair { first_value: 10, second_value: 20 };
    comparable_pair.find_and_display_maximum();

    // 3. 运算符重载示例
    println!("\n3. 运算符重载示例:");
    let first_vector = TwoDVector::new(3.0, 4.0);
    let second_vector = TwoDVector::new(1.0, 2.0);
    let sum_vector = first_vector + second_vector;
    let scaled_vector = sum_vector * 2.0;

    println!("first_vector = {}", first_vector);
    println!("second_vector = {}", second_vector);
    println!("first_vector + second_vector = {}", sum_vector);
    println!("(first_vector + second_vector) * 2 = {}", scaled_vector);
    println!("scaled_vector 的长度: {:.2}", scaled_vector.calculate_magnitude());

    // 4. Summarizable trait 示例
    println!("\n4. Summarizable trait 示例:");
    let news_article = NewsArticle {
        article_title: "Rust Trait 学习指南".to_string(),
        article_content: "Trait 是 Rust 中定义共享行为的强大工具...".to_string(),
    };

    process_generic_item(news_article.clone());
    process_trait_object_item(&news_article);
}