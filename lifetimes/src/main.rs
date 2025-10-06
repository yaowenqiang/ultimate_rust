// 函数生命周期示例
// 这个函数演示了如何在函数签名中使用生命周期参数
//
// 参数说明：
// - i: 第一个 i32 的引用，生命周期为 'a
// - j: 第二个 i32 的引用，生命周期为 'a
// - 返回值: 返回一个 i32 的引用，生命周期也为 'a
//
// 生命周期 'a 确保返回的引用在传入的两个引用都有效时才有效
// 这是 Rust 借用检查器的核心概念，防止悬垂指针
//
// 文档链接：https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-function-signatures
fn borrow<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
    // 简单返回第一个参数
    // 由于返回值的生命周期是 'a，这保证了返回的引用至少和输入参数一样长
    i
}

// Cat 结构体：表示一只猫，包含名字
// 这个结构体拥有一个 String 类型的数据
struct Cat(String);

// CatFeeder 结构体：演示结构体中的生命周期参数
//
// 这个结构体包含一个对 Cat 的可变引用，生命周期参数 'a 确保了
// CatFeeder 实例不能比它引用的 Cat 活得更久
//
// 这是 Rust 中防止悬垂引用的重要机制
//
// 文档链接：https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-structs
struct CatFeeder<'a> {
    // cat 是对 Cat 实例的可变引用，生命周期为 'a
    // 这意味着 CatFeeder 不能比它引用的 Cat 活得更久
    cat: &'a mut Cat,
}

// Cat 结构体的方法实现
impl Cat {
    // feed 方法：给猫喂食
    // 这是一个可变方法（&mut self），它会修改猫的状态
    // 将猫的名字后面添加 " feeded" 来表示已经喂食
    fn feed(&mut self) {
        // 修改猫的名字，添加喂食标记
        // self.0 访问结构体的第一个（也是唯一一个）字段
        self.0 = format!("{} feeded", self.0);
    }
}

// CatFeeder 结构体的方法实现，带生命周期参数 'a
impl<'a> CatFeeder<'a> {
    // feed 方法：通过 CatFeeder 给猫喂食
    // 这个方法调用它所引用的 Cat 的 feed 方法
    //
    // 注意：这里的 &mut self 表示我们可以修改 CatFeeder，
    // 但实际上我们没有修改 CatFeeder 本身，而是修改它引用的 Cat
    fn feed(&mut self) {
        // 调用所引用的 Cat 的 feed 方法
        // 由于 cat 是 &mut Cat，我们可以调用 Cat 的可变方法
        self.cat.feed();
    }
}

// ==============================================
// 生命周期省略规则示例
// ==============================================

// 函数1：没有明确的生命周期参数，但可以正常工作
// 这是生命周期省略规则的应用
//
// 省略规则：
// 1. 每个输入参数都有自己的生命周期
// 2. 如果只有一个输入生命周期参数，该生命周期赋给所有输出生命周期参数
// 3. 如果有多个输入生命周期参数，但其中一个是 &self 或 &mut self，
//    self 的生命周期赋给所有输出生命周期参数
//
// 文档链接：https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision-rules
fn first_word(s: &str) -> &str {
    // 根据省略规则2，这个函数实际上等同于：
    // fn first_word<'a>(s: &'a str) -> &'a str
    // 但 Rust 允许我们省略生命周期参数
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}

// 函数2：返回引用，但需要明确的生命周期参数
// 因为有两个输入参数，不能应用省略规则
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 返回两个字符串切片中较长的一个
    // 返回值的生命周期必须与两个输入参数中的较小生命周期一致
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ==============================================
// 静态生命周期示例
// ==============================================

// 静态生命周期 'static 表示程序整个运行期间都有效的引用
// 常见于字符串字面量和全局变量
//
// 文档链接：https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#the-static-lifetime
fn get_static_str() -> &'static str {
    // 字符串字面量自动具有 'static 生命周期
    // 因为它们直接存储在程序的二进制文件中
    "Hello, Static World!"
}

// 全局常量，具有静态生命周期
const GLOBAL_MESSAGE: &str = "This is a global static message";

// ==============================================
// 多个生命周期参数示例
// ==============================================

// 结构体包含多个不同生命周期的引用
struct ImportantExcerpt<'a, 'b> {
    part: &'a str,        // 引用的一部分，生命周期为 'a
    author: &'b str,      // 作者信息，生命周期为 'b
}

impl<'a, 'b> ImportantExcerpt<'a, 'b> {
    // 方法可能需要额外的生命周期参数
    fn announce_and_return_part<'c>(&'c self, announcement: &'c str) -> &'c str {
        // 这里的生命周期 'c 表示方法调用的持续时间
        println!("Announcement: {}", announcement);
        self.part  // 但返回的是 'a 生命周期的数据
    }
}

// ==============================================
// 生命周期子类型化示例
// ==============================================

// 演示较长的生命周期可以转换为较短的生命周期
fn coerce_lifetime<'a>(long_lived: &'static str) -> &'a str {
    // 'static 可以强制转换为任何更短的生命周期 'a
    // 这被称为生命周期子类型化
    long_lived
}

// ==============================================
// 高级生命周期示例：Context 结构体
// ==============================================

// 一个更复杂的例子，演示如何在实际应用中使用生命周期
struct Context<'a> {
    // Context 包含对一些数据的引用
    data: &'a str,
}

struct Parser<'a> {
    // Parser 依赖于 Context，所以它不能比 Context 活得更久
    context: &'a Context<'a>,
}

impl<'a> Parser<'a> {
    // 解析方法，返回解析结果
    fn parse(&self) -> Result<&str, &str> {
        if self.context.data.is_empty() {
            Err("Empty data")
        } else {
            Ok(&self.context.data[0..1]) // 返回第一个字符
        }
    }

    // 获取完整的上下文数据
    fn get_data(&self) -> &'a str {
        self.context.data
    }
}

// 主函数：演示生命周期概念的实际应用
fn main() {
    // ==============================================
    // 第一部分：函数生命周期示例
    // ==============================================

    let n = 12;                                    // n 是一个 i32，存储在栈上
    let borrowed_n = &n;                           // borrowed_n 是对 n 的不可变引用

    // 测试 borrow 函数
    // borrow(&n, &n);                            // 这行代码被注释掉了，但也可以工作
    borrow(&n, borrowed_n);                      // 传入两个引用，它们的生命周期兼容

    // ==============================================
    // 第二部分：结构体生命周期示例
    // ==============================================

    // 创建一个可变的 Cat 向量
    // 这些 Cat 实例拥有它们的名字字符串
    let mut cats = vec![Cat("Cat1".to_string()), Cat("Cat2".to_string())];

    // 创建 CatFeeder 向量
    // 每个 CatFeeder 都包含对一个 Cat 的可变引用
    let mut feeders = Vec::new();
    for cat in cats.iter_mut() {                  // 获取每个 Cat 的可变引用
        feeders.push(CatFeeder { cat });          // 创建 CatFeeder，存储对 Cat 的引用
    }

    // 使用每个 CatFeeder 给对应的猫喂食
    // 这里演示了生命周期如何确保引用的有效性
    feeders.iter_mut().for_each(|f| f.feed());

    // ==============================================
    // 第三部分：生命周期省略规则示例
    // ==============================================

    let sentence = String::from("Hello world programming");
    let first = first_word(&sentence);  // 省略生命周期参数
    println!("First word: {}", first);

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(&string1, string2);  // 需要明确的生命周期参数
    println!("Longest string: {}", result);

    // ==============================================
    // 第四部分：静态生命周期示例
    // ==============================================

    let static_ref = get_static_str();
    println!("Static string: {}", static_ref);
    println!("Global message: {}", GLOBAL_MESSAGE);

    // ==============================================
    // 第五部分：生命周期子类型化示例
    // ==============================================

    let short_lived_str = String::from("I'm short lived");
    let coerced: &str = coerce_lifetime("I can live forever!");
    println!("Coerced static string: {}", coerced);

    // ==============================================
    // 第六部分：多个生命周期参数示例
    // ==============================================

    let novel = String::from("Call me Ishmael. Some years ago...");
    let author_name = "Herman Melville";

    {
        let excerpt = ImportantExcerpt {
            part: &novel[..10],  // "Call me Is"
            author: author_name,
        };

        println!("Excerpt part: {}", excerpt.part);
        println!("Excerpt author: {}", excerpt.author);
    }

    // ==============================================
    // 第七部分：高级生命周期示例（Context 和 Parser）
    // ==============================================

    let data_string = String::from("parse me");
    let context = Context { data: &data_string };
    let parser = Parser { context: &context };

    match parser.parse() {
        Ok(result) => println!("Parsed: {}", result),
        Err(e) => println!("Parse error: {}", e),
    }

    println!("Parser data: {}", parser.get_data());

    // ==============================================
    // 总结
    // ==============================================
    //
    // 本示例展示了 Rust 生命周期的核心概念：
    // 1. 函数生命周期参数确保返回引用的有效性
    // 2. 结构体生命周期注解防止悬垂引用
    // 3. 生命周期省略规则简化代码编写
    // 4. 静态生命周期用于全局数据
    // 5. 生命周期子类型化提供灵活性
    // 6. 多个生命周期参数处理复杂场景
    // 7. 借用检查器在编译时保证内存安全
    //
    // 相关学习资源：
    // - Rust Book 第10章：https://doc.rust-lang.org/book/ch10-00-generic-types-traits-and-lifetimes.html
    // - Rust by Example 生命周期：https://doc.rust-lang.org/rust-by-example/scope/lifetime.html
    // - Rust Reference：https://doc.rust-lang.org/reference/lifetime-elision.html
    // - Rustonomicon：https://doc.rust-lang.org/nomicon/lifetimes.html
}
