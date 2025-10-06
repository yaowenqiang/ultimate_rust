// 生命周期错误示例
//
// 这个文件包含了一些常见的生命周期错误示例。
// 这些代码被注释掉了，因为它们无法通过编译。
// 解除注释可以看到具体的编译错误信息。
//
// 这些错误示例帮助理解：
// 1. 为什么需要生命周期
// 2. 借用检查器如何工作
// 3. 常见的生命周期陷阱

// ==============================================
// 错误示例1：返回悬垂引用
// ==============================================

// 错误：试图返回函数局部变量的引用
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s  // 错误：s 将在函数结束时被销毁
// }

// 正确的版本：返回所有权
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // 正确：转移所有权
}

// ==============================================
// 错误示例2：生命周期不匹配
// ==============================================

// 错误：试图返回不同生命周期的引用
// fn wrong_longest<'a>(x: &'a str, y: &str) -> &'a str {
//     let result = String::from("new string");
//     if x.len() > y.len() {
//         x
//     } else {
//         // 错误：试图返回局部变量 result 的引用
//         &result
//     }
// }

// ==============================================
// 错误示例3：结构体中的生命周期问题
// ==============================================

struct WrongRef<'a> {
    x: &'a i32,
}

// 错误：试图创建包含悬垂引用的结构体
// fn create_wrong_ref() -> WrongRef {
//     let x = 5;
//     WrongRef { x: &x }  // 错误：x 将在函数结束时被销毁
// }

// ==============================================
// 错误示例4：借用检查器错误
// ==============================================

// 错误：同时存在可变和不可变引用
// fn borrow_checker_error() {
//     let mut s = String::from("hello");
//     let r1 = &s;        // 不可变引用
//     let r2 = &s;        // 另一个不可变引用
//     let r3 = &mut s;    // 错误：不能在存在不可变引用时创建可变引用
//
//     println!("{}, {}, and {}", r1, r2, r3);
// }

// 正确的版本：确保引用的作用域不重叠
fn borrow_checker_correct() {
    let mut s = String::from("hello");
    {
        let r1 = &s;        // 不可变引用
        let r2 = &s;        // 另一个不可变引用
        println!("{} and {}", r1, r2);
//         r1 和 r2 在这里离开作用域
    }

    let r3 = &mut s;    // 现在可以创建可变引用
    println!("{}", r3);
}

// ==============================================
// 错误示例5：生命周期省略失败
// ==============================================

// 错误：编译器无法推断生命周期
// fn ambiguous_lifetime(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
//     // 错误：编译器不知道返回值应该与哪个参数的生命周期绑定
// }

// 正确的版本：明确指定生命周期
fn explicit_lifetime<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
    // 正确：明确指定返回值的生命周期与输入参数相同
}

// ==============================================
// 测试函数
// ==============================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_dangle() {
        let s = no_dangle();
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_borrow_checker_correct() {
        borrow_checker_correct();
    }

    #[test]
    fn test_explicit_lifetime() {
        let s1 = "short";
        let s2 = "much longer string";
        let result = explicit_lifetime(s1, s2);
        assert_eq!(result, s2);
    }
}