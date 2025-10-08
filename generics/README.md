# Rust 泛型 (Generics) 学习示例

这个项目包含了 Rust 泛型 (Generics) 的全面学习示例，从基础概念到高级用法都有详细的中文注释和实用示例。

## 📚 项目结构

```
generics/
├── src/
│   └── main.rs              # 泛型概念和示例代码
├── Cargo.toml
└── README.md               # 本文件
```

## 🎯 学习内容

### 泛型基础概念

1. **泛型结构体 (Generic Structs)**
   - `GroupedHashMap<K, V>`: 泛型哈希映射桶，支持多值存储
   - 多个泛型参数的使用和约束
   - where 子句约束，提高代码可读性
   - 自定义迭代器实现 `HashMapBucketIter`

2. **泛型函数 (Generic Functions)**
   - `display_two_values<T, U>`: 多类型参数函数演示
   - `compare_and_display<T>`: 单类型参数比较函数
   - `calculate_sine()`: impl Trait 语法简化函数签名
   - 泛型约束和 trait bounds 的实际应用

3. **类型转换 (Type Conversion)**
   - `From` trait 实现和 `Into` trait 使用
   - 角度单位转换系统 (度数 ↔ 弧度)
   - 类型转换链演示和实际应用场景
   - 三角函数计算和数学运算示例

4. **自定义迭代器 (Custom Iterator)**
   - `HashMapBucketIter<'a, K, V>`: 生命周期与泛型结合
   - 复杂迭代逻辑实现：扁平化遍历嵌套结构
   - 迭代器链式操作：filter、map、collect 等
   - 惰性求值和性能优势演示

5. **高级泛型特性**
   - 泛型常量和类型别名
   - 复杂的 trait bounds 和多重约束
   - 浮点数精度比较和解决方案
   - Unicode 字符串比较和中文字符处理

## 🚀 运行项目

```bash
# 编译并运行
cargo run

# 只编译检查
cargo check

# 生成优化版本
cargo build --release
```

## 📖 示例输出

运行 `cargo run` 将展示以下概念的详细演示：

### 🔰 基础泛型概念
1. **泛型函数示例** - 多类型参数的函数调用，包括混合类型
2. **全面比较示例** - 整数、浮点数、字符串、字符、Unicode 字符比较
3. **浮点数精度专题** - 演示浮点数比较陷阱和解决方案

### 🔄 类型转换系统
4. **角度转换示例** - 完整的度数 ↔ 弧度转换系统
5. **类型转换链演示** - From/Into trait 的实际应用
6. **三角函数计算** - 数学运算中的类型转换应用

### 📊 高级特性演示
7. **泛型结构体示例** - 类型安全的多值哈希容器
8. **自定义迭代器演示** - 复杂迭代逻辑的完整实现
9. **迭代器链式操作** - filter、map、collect 等函数式编程
10. **性能对比演示** - 迭代器 vs 传统循环的性能分析

### 🎯 实际应用场景
- 常用角度对照表 (0°, 30°, 45°, 60°, 90°, 180°, 270°, 360°)
- 三角函数值计算和显示
- 技术栈分类和统计 (编程语言、数据库、框架)
- Unicode 字符和中文字符串处理

## 🔗 相关文档

### 官方文档
- [Rust Book - 泛型数据类型](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Rust by Example - 泛型](https://doc.rust-lang.org/rust-by-example/generics.html)
- [高级 Trait 和生命周期](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html)

### 具体主题文档
- [泛型约束和 where 子句](https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods)
- [From trait 文档](https://doc.rust-lang.org/std/convert/trait.From.html)
- [Into trait 文档](https://doc.rust-lang.org/std/convert/trait.Into.html)
- [ToString trait](https://doc.rust-lang.org/string/trait.ToString.html)
- [Debug trait](https://doc.rust-lang.org/std/fmt/trait.Debug.html)

## 💡 学习要点

### 1. 泛型的优势
- **类型安全**: 编译时检查类型错误
- **代码复用**: 一套代码适用于多种类型
- **性能优化**: 编译时单态化，零成本抽象

### 2. 泛型语法要点
- **泛型参数**: 使用 `<T>`, `<K, V>` 等语法
- **约束条件**: 使用 `where` 子句或 `:` 语法
- **生命周期**: 泛型可以与生命周期结合使用

### 3. impl Trait 语法
- **函数参数**: 简化泛型函数签名
- **返回值**: 返回实现了特定 trait 的类型
- **限制**: 不能用于多个返回路径

### 4. 类型转换
- **From trait**: 定义如何从其他类型创建此类型
- **Into trait**: From trait 的反向操作
- **自动实现**: 实现了 From 的类型自动获得 Into

## 🎨 代码特色

### 优化的命名约定
- `GroupedHashMap`: 描述性的结构体名
- `AngleDegrees`, `AngleRadians`: 清晰的类型名
- `display_two_values`: 动词开头的函数名
- `angle_in_degrees`: 描述性的变量名
- `HashMapBucketIter`: 自定义迭代器名
- `create_bucket_iterator`: 动词开头的迭代器创建方法
- `current_map_entry`: 描述性的迭代器状态字段名

### 详细的中文注释
- 每个概念都有详细的中文解释
- 包含示例代码和使用说明
- 注释中包含最佳实践提示

### 实用示例
- 角度转换的完整实现
- 多类型容器的设计
- 类型安全的比较函数

## 🔄 高级概念演示

### 1. 泛型约束
```rust
where
    T: ToString + Debug,  // 多重约束
    K: Eq + Hash + Debug, // 复杂约束组合
```

### 2. 类型别名
```rust
type StringIntMap = GroupedHashMap<String, i32>;
```

### 3. 泛型常量
```rust
const BUFFER_SIZE: usize = 1024;
let buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
```

## 🎯 实际应用场景

1. **集合类型**: 创建类型安全的数据容器
2. **API 设计**: 提供灵活且类型安全的接口
3. **算法实现**: 编写适用于多种类型的算法
4. **类型系统**: 构建强类型的抽象层

## 🛠️ 最佳实践

1. **命名约定**:
   - 使用 `T`, `U`, `V` 表示泛型类型参数
   - 使用描述性的名称更好，如 `K` 表示键，`V` 表示值

2. **约束使用**:
   - 优先使用 `where` 子句，使代码更清晰
   - 只在必要时添加约束，保持泛型的灵活性

3. **性能考虑**:
   - 泛型在编译时单态化，性能与手写代码相同
   - 避免过度使用泛型导致代码膨胀

## 🤝 贡献

欢迎提交 Issue 和 Pull Request 来改进这个学习项目！

## 📄 许可证

本项目采用 MIT 许可证。