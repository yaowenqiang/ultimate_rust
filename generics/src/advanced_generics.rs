/*
 * 高级泛型概念示例
 *
 * 本文件展示更复杂的泛型用法和高级概念
 */

use std::fmt::{Debug, Display};
use std::collections::HashMap;

// ==================== 1. 泛型关联类型 (Generic Associated Types) ====================

/// 泛型迭代器示例
struct GenericIterator<T> {
    items: Vec<T>,
    current_index: usize,
}

impl<T> GenericIterator<T> {
    fn new(items: Vec<T>) -> Self {
        GenericIterator {
            items,
            current_index: 0,
        }
    }
}

impl<T: Clone> Iterator for GenericIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.items.len() {
            let item = self.items[self.current_index].clone();
            self.current_index += 1;
            Some(item)
        } else {
            None
        }
    }
}

// ==================== 2. 泛型生命周期 ====================

/// 带有生命周期参数的泛型结构体
struct LifetimedReference<'a, T: 'a> {
    reference: &'a T,
}

impl<'a, T: 'a> LifetimedReference<'a, T> {
    fn create(reference: &'a T) -> Self {
        LifetimedReference { reference }
    }

    fn get_reference(&self) -> &T {
        self.reference
    }
}

// ==================== 3. 条件泛型实现 (Conditional Implementation) ====================

/// 泛型结构体，只有当 T 实现了特定 trait 时才有某些方法
struct ConditionalGeneric<T> {
    value: T,
}

impl<T> ConditionalGeneric<T> {
    fn new(value: T) -> Self {
        ConditionalGeneric { value }
    }

    // 基本方法，对所有 T 都可用
    fn get_value(&self) -> &T {
        &self.value
    }
}

// 只有当 T 实现了 Display 时才有这个方法
impl<T: Display> ConditionalGeneric<T> {
    fn display_value(&self) -> String {
        format!("值: {}", self.value)
    }
}

// 只有当 T 实现了 Clone 时才有这个方法
impl<T: Clone> ConditionalGeneric<T> {
    fn clone_value(&self) -> T {
        self.value.clone()
    }
}

// ==================== 4. 泛型常量和类型级编程 ====================

/// 泛型数组包装器，使用常量泛型参数
struct GenericArray<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> GenericArray<T, N> {
    fn new(data: [T; N]) -> Self {
        GenericArray { data }
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    fn len(&self) -> usize {
        N
    }
}

impl<T: Display, const N: usize> Display for GenericArray<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

// ==================== 5. 复杂泛型约束示例 ====================

/// 多个泛型参数和复杂约束的函数
fn complex_generic_function<
    T: Clone + PartialEq + Debug,
    U: From<T> + Display,
    V: Into<HashMap<String, U>>
>(
    source: &T,
    transformer: V,
) -> Result<String, String> {
    // 克隆源值
    let cloned_value = source.clone();

    // 转换为目标类型
    let transformed_value: U = U::from(cloned_value);

    // 转换为 HashMap
    let mut map: HashMap<String, U> = transformer.into();
    map.insert("result".to_string(), transformed_value);

    // 格式化输出
    Ok(format!("映射包含 {} 个项目", map.len()))
}

// ==================== 6. 泛型常量示例 ====================

/// 泛型常量使用示例
fn demonstrate_generic_constants() {
    // 使用泛型常量创建固定大小的缓冲区
    const BUFFER_SIZE: usize = 1024;
    let mut generic_buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    generic_buffer[0] = 42;

    println!("泛型常量缓冲区大小: {}", BUFFER_SIZE);
    println!("缓冲区第一个元素: {}", generic_buffer[0]);
}

// ==================== 7. 类型级编程示例 ====================

/// 类型级编程示例 - 使用类型表示状态
struct Empty;
struct Filled<T> {
    value: T,
}

/// 状态容器
struct StateContainer<T, State> {
    data: Option<T>,
    _state: PhantomData<State>,
}

use std::marker::PhantomData;

impl<T> StateContainer<T, Empty> {
    fn new() -> Self {
        StateContainer {
            data: None,
            _state: PhantomData,
        }
    }

    fn fill(self, value: T) -> StateContainer<T, Filled<T>> {
        StateContainer {
            data: Some(value),
            _state: PhantomData,
        }
    }
}

impl<T> StateContainer<T, Filled<T>> {
    fn get_value(&self) -> &T {
        self.data.as_ref().unwrap()
    }
}

// ==================== 8. 演示函数 ====================

/// 演示各种高级泛型概念
pub fn demonstrate_advanced_generics() {
    println!("=== 高级泛型概念演示 ===\n");

    // 1. 泛型迭代器示例
    println!("1. 泛型迭代器示例:");
    let numbers = vec![1, 2, 3, 4, 5];
    let mut iterator = GenericIterator::new(numbers);

    print!("迭代器输出: ");
    while let Some(num) = iterator.next() {
        print!("{} ", num);
    }
    println!("\n");

    // 2. 生命周期泛型示例
    println!("2. 生命周期泛型示例:");
    let message = "Hello, Rust!";
    let message_ref = LifetimedReference::create(&message);
    println!("引用的内容: {}", message_ref.get_reference());

    // 3. 条件泛型实现示例
    println!("\n3. 条件泛型实现示例:");
    let displayable = ConditionalGeneric::new(42);
    println!("显示值: {}", displayable.display_value());
    println!("克隆值: {}", displayable.clone_value());

    let string_container = ConditionalGeneric::new("字符串".to_string());
    println!("字符串显示: {}", string_container.display_value());
    println!("字符串克隆: {}", string_container.clone_value());

    // 4. 泛型数组示例
    println!("\n4. 泛型数组示例:");
    let number_array = GenericArray::new([1, 2, 3, 4, 5]);
    println!("数字数组: {}", number_array);
    println!("数组长度: {}", number_array.len());
    println!("第三个元素: {:?}", number_array.get(2));

    let string_array = GenericArray::new(["苹果", "香蕉", "橙子"]);
    println!("字符串数组: {}", string_array);

    // 5. 复杂泛型约束示例
    println!("\n5. 复杂泛型约束示例:");
    let source_value = 100;
    let empty_map: HashMap<String, i32> = HashMap::new();

    match complex_generic_function(&source_value, empty_map) {
        Ok(result) => println!("复杂函数结果: {}", result),
        Err(error) => println!("错误: {}", error),
    }

    // 6. 泛型常量示例
    println!("\n6. 泛型常量示例:");
    demonstrate_generic_constants();

    // 7. 类型级编程示例
    println!("\n7. 类型级编程示例:");
    let empty_container: StateContainer<i32, Empty> = StateContainer::new();
    let filled_container = empty_container.fill(42);
    println!("状态容器中的值: {}", filled_container.get_value());

    println!("\n=== 高级泛型概念演示完成 ===");
}