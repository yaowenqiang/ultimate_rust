/*
 * Rust 构造函数和析构函数深入学习示例
 *
 * 本项目展示了 Rust 中的构造函数模式、析构函数（Drop trait）、
 * 内存管理和智能指针的概念，这是理解 Rust 资源管理的关键。
 *
 * 📚 官方文档链接：
 *
 * 🔰 基础概念
 * 1. Rust Book - 结构体:
 *    https://doc.rust-lang.org/book/ch05-01-defining-structs.html
 *
 * 2. Rust Book - 方法语法:
 *    https://doc.rust-lang.org/book/ch05-03-method-syntax.html
 *
 * 3. Rust Book - Drop trait:
 *    https://doc.rust-lang.org/book/ch15-03-drop.html
 *
 * ⚙️ 内存管理
 * 4. std::alloc 模块:
 *    https://doc.rust-lang.org/std/alloc/index.html
 *
 * 5. Box 智能指针:
 *    https://doc.rust-lang.org/std/boxed/struct.Box.html
 *
 * 6. 自定义智能指针:
 *    https://doc.rust-lang.org/book/ch15-01-box.html
 *
 * 🚀 高级概念
 * 7. RAII (Resource Acquisition Is Initialization):
 *    https://doc.rust-lang.org/book/ch15-03-drop.html#the-drop-trait-runs-at-the-end-of-a-scope
 *
 * 8. 不安全 Rust:
 *    https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
 *
 * 9. 所有权和移动语义:
 *    https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
 */

use std::alloc::{Layout, alloc, dealloc}; // 内存分配和释放模块

// MyStruct 结构体：演示基本的构造函数和析构函数
//
// 这个结构体包含一个整数值，用于演示：
// 1. 构造函数模式（new 方法）
// 2. 析构函数模式（Drop trait 实现）
// 3. 所有权的移动语义
struct MyStruct {
    n: i32, // 存储整数值的字段
}

// HasDroppables 结构体：演示复合类型的析构行为
//
// 这个结构体包含一个 MyStruct 实例，用于演示：
// 1. 结构体字段的析构顺序
// 2. 嵌套资源的自动清理
struct HasDroppables {
    x: MyStruct, // 包含一个有析构函数的字段
}

// MyStruct 的实现块
impl MyStruct {
    // 构造函数：创建新的 MyStruct 实例
    //
    // 这是 Rust 中常见的构造函数模式：
    // - 使用关联函数（静态方法）作为构造函数
    // - 命名约定：通常命名为 "new"
    // - 返回 Self 类型（当前类型）
    //
    // 参数:
    //   n: 要存储的整数值
    //
    // 返回值:
    //   新创建的 MyStruct 实例
    //
    // 示例:
    //   let my_struct = MyStruct::new(42);
    pub fn new(n: i32) -> Self {
        println!("constructing {n}"); // 打印构造信息
        Self { n } // 使用结构体字面量创建实例
    }
}

// 为 MyStruct 实现 Drop trait，实现析构函数
//
// Drop trait 允许我们在实例被销毁时执行自定义清理代码。
// 这是 Rust RAII (Resource Acquisition Is Initialization) 模式的核心。
//
// 析构函数会自动在以下情况下调用：
// 1. 变量离开作用域
// 2. 所有权被移动且不再使用
// 3. 程序结束时
//
// 注意：不能手动调用 drop 方法，可以使用 std::mem::drop 函数
impl Drop for MyStruct {
    // 析构函数：在实例被销毁时自动调用
    //
    // 这个方法演示了资源的自动清理：
    // - 当 MyStruct 实例离开作用域时自动调用
    // - 用于释放资源、关闭文件、网络连接等
    // - 按照与创建相反的顺序调用（后创建的先销毁）
    fn drop(&mut self) {
        println!("dropping {}", self.n); // 打印析构信息
    }
}

// 自定义智能指针结构体
//
// 这个结构体演示了如何创建一个简单的智能指针，类似于 Box<T>：
// - 手动管理内存分配和释放
// - 使用泛型支持任何类型
// - 实现自动内存管理
//
// 字段说明：
// - ptr: 分配的内存块指针（用于释放）
// - data: 类型化的数据指针（用于访问）
// - layout: 内存布局信息（用于正确释放）
struct SmartPointer<T> {
    ptr: *mut u8,      // 指向分配的内存块的原始指针
    data: *mut T,       // 指向类型化数据的指针
    layout: Layout,     // 内存布局信息，包含大小和对齐方式
}

// SmartPointer 的泛型实现
impl<T> SmartPointer<T> {
    // 创建新的智能指针实例
    //
    // 这个方法演示了手动内存分配：
    // - 使用 std::alloc::alloc 分配内存
    // - 保存内存布局信息以便后续释放
    // - 返回包含原始指针和类型化指针的实例
    //
    // 注意：这个方法分配了内存但不会初始化数据！
    // 使用 set 方法来设置数据值
    pub fn new() -> SmartPointer<T> {
        println!("Allocating memory for SmartPointer");
        unsafe {
            // 获取类型 T 的内存布局
            let layout = Layout::new::<T>();
            // 分配内存
            let ptr: *mut u8 = alloc(layout);
            SmartPointer {
                ptr,
                data: ptr as *mut T,
                layout,
            }
        }
    }

    // 设置智能指针中的值
    //
    // 将给定的值写入分配的内存中
    //
    // 参数:
    //   val: 要存储的值
    fn set(&mut self, val: T) {
        unsafe {
            // 将值写入分配的内存位置
            *self.data = val;
        }
    }

    // 获取智能指针中值的引用
    //
    // 返回存储值的不可变引用
    // 注意：如果数据未初始化，这可能导致未定义行为
    fn get(&self) -> &T {
        unsafe {
            // 将裸指针转换为引用
            // 注意：这里假设数据已经被正确初始化
            self.data.as_ref().unwrap()
        }
    }
}

// 为 SmartPointer 实现 Drop trait
impl<T> Drop for SmartPointer<T> {
    // 析构函数：自动释放分配的内存
    //
    // 这确保了当 SmartPointer 离开作用域时，
    // 分配的内存会被正确释放，防止内存泄漏
    fn drop(&mut self) {
        println!("Deallocating memory for SmartPointer");
        unsafe {
            // 释放之前分配的内存
            dealloc(self.ptr, self.layout);
        }
    }
}

// 演示所有权的移动语义
//
// 这个函数接收一个 MyStruct 参数，展示了所有权的转移：
// - 当函数被调用时，x 的所有权从调用者移动到函数内
// - 函数返回后，x 被销毁（drop）
// - 调用者不能再使用原来的变量
fn move_me(x: MyStruct) {
    // 函数结束时，x 会自动被销毁
    // 调用 MyStruct 的 drop 实现
}

// 主函数：演示构造函数、析构函数和智能指针的使用
fn main() {
    println!("=== 基本构造和析构演示 ===");

    // 创建 MyStruct 实例 x
    let x = MyStruct::new(5);

    // 创建内部作用域
    {
        // 创建 MyStruct 实例 y
        // y 只在这个作用域内有效
        let y = MyStruct::new(2);
        // 作用域结束时，y 被销毁
    }

    // 将 x 的所有权移动到函数中
    move_me(x);
    // x 在这里不能再使用，因为所有权已经移动
    println!("back from function");

    // 创建包含可析构字段的结构体
    let has_drop = HasDroppables {
        x: MyStruct::new(10),
    };
    // has_drop 将在 main 函数结束时被销毁

    println!("=== 自定义智能指针演示 ===");
    println!("Ending the main function");

    // 创建自定义智能指针
    let mut my_num: SmartPointer<i32> = SmartPointer::<i32>::new();
    my_num.set(1);
    println!("my_num = {}", my_num.get());
    // my_num 离开作用域时自动释放内存

    // 对比：使用标准库的 Box 智能指针
    println!("\n=== 标准库 Box 智能指针对比 ===");
    let my_num = Box::new(12);
    println!("my_num = {}", my_num);
    // Box 也会在离开作用域时自动释放内存

    // 程序结束时，所有变量按相反顺序被销毁：
    // 1. Box<i32>
    // 2. SmartPointer<i32>
    // 3. has_drop.x (MyStruct)
    // 4. has_drop (HasDroppables)
}
