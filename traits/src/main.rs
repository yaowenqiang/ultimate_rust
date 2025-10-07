// 导入标准库模块
use std::any::Any;           // 运行时类型识别和向下转换
use std::fmt;                // 格式化输出
use std::fmt::Debug;         // 调试格式化 trait
use std::ops::Add;           // 加法运算符重载 trait

// 导入高级 trait 示例
mod advanced_traits;
use advanced_traits::demonstrate_advanced_traits;

/*
 * Rust Trait (特征/接口) 学习示例
 *
 * 📚 相关文档链接：
 *
 * 1. Rust 官方文档 - Traits:
 *    https://doc.rust-lang.org/rust-by-example/trait.html
 *
 * 2. Rust Book - Trait 定义和实现:
 *    https://doc.rust-lang.org/book/ch10-02-traits.html
 *
 * 3. Trait 对象和动态分发:
 *    https://doc.rust-lang.org/book/ch17-02-trait-objects.html
 *
 * 4. 运算符重载:
 *    https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
 *
 * 5. 常用的派生 trait:
 *    - Debug: https://doc.rust-lang.org/std/fmt/trait.Debug.html
 *    - Clone: https://doc.rust-lang.org/std/clone/trait.Clone.html
 *    - Copy: https://doc.rust-lang.org/std/marker/trait.Copy.html
 *    - PartialEq: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
 *    - Display: https://doc.rust-lang.org/std/fmt/trait.Display.html
 *
 * 6. 运算符重载相关 trait:
 *    - Add: https://doc.rust-lang.org/std/ops/trait.Add.html
 *    - 更多运算符: https://doc.rust-lang.org/std/ops/index.html
 *
 * 🎯 学习要点：
 * - Trait 定义了共享行为
 * - Trait 可以有默认实现
 * - 支持泛型约束和 trait bounds
 * - 可以进行运算符重载
 * - 支持 trait 对象的动态分发
 * - 可以使用 derive 自动实现常见 trait
 */

// 定义一个二维坐标点结构体
// struct 用于组织相关的坐标数据
#[derive(Debug, Clone, Copy, PartialEq)]  // 添加更多 trait
struct CoordinatePoint {
    x_coordinate: f32,  // x 坐标，f32 类型是 32 位浮点数
    y_coordinate: f32,  // y 坐标
}

// 为 CoordinatePoint 实现 Display trait，提供用户友好的显示格式
impl fmt::Display for CoordinatePoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({}, {})", self.x_coordinate, self.y_coordinate)
    }
}

// 为 CoordinatePoint 实现加法运算符重载
// 这是一个 trait 实现的示例，让自定义类型支持 + 运算符
impl Add for CoordinatePoint {
    // 关联类型：定义加法运算的结果类型
    type Output = CoordinatePoint;

    // add 方法的实现：定义两个 CoordinatePoint 相加的行为
    fn add(self, other_point: CoordinatePoint) -> Self::Output {
        CoordinatePoint {
            x_coordinate: self.x_coordinate + other_point.x_coordinate,  // x 坐标相加
            y_coordinate: self.y_coordinate + other_point.y_coordinate,  // y 坐标相加
        }
    }
}

// 定义 Speakable trait（可说话特征/接口）
// trait 定义了共享行为，类似于其他语言中的接口
// : Debug 表示任何实现 Speakable 的类型也必须实现 Debug
trait Speakable: Debug {
    // 定义方法签名，所有实现 Speakable 的类型必须提供这个方法
    fn make_sound(&self);
}

// 定义 HouseCat（家猫）结构体
// #[derive(Debug, Clone, PartialEq)] 自动为 HouseCat 实现多个 trait
#[derive(Debug, Clone, PartialEq)]
struct HouseCat;

// 定义 DomesticDog（家犬）结构体
#[derive(Debug, Clone, PartialEq)]
struct DomesticDog;

// 为 Speakable 添加一个新的类型：WildBird（野鸟）
#[derive(Debug, Clone, PartialEq)]
struct WildBird {
    bird_species: String,  // 鸟的种类
}

// 为 WildBird 实现 Speakable trait
impl Speakable for WildBird {
    fn make_sound(&self) {
        println!("Chirp! I'm a {}", self.bird_species);
    }
}

// 为 WildBird 实现 Display trait
impl fmt::Display for WildBird {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "WildBird({})", self.bird_species)
    }
}

// 为 HouseCat 实现 Speakable trait
impl Speakable for HouseCat {
    // 实现 make_sound 方法
    fn make_sound(&self) {
        // 喵喵叫
        println!("Meow");
    }
}

// 为 DomesticDog 实现 Speakable trait
impl Speakable for DomesticDog {
    fn make_sound(&self) {
        // 汪汪叫
        println!("Woof");
    }
}

// 注意：由于使用了 #[derive(Debug)]，我们不再需要手动实现 Debug trait
// 如果要自定义 Debug 格式，可以移除 derive(Debug) 并手动实现

// 使用 trait bounds 的函数示例
// &impl Speakable 是 Rust 2015 引入的简写语法，称为 "impl Trait"
// 它表示接受任何实现了 Speakable trait 的类型的引用
fn make_animal_speak_twice(animal: &impl Speakable) {
    animal.make_sound();  // 第一次调用
    animal.make_sound();  // 第二次调用
    println!("{animal:?}");  // 使用 Debug trait 打印动物信息
}

// 返回 impl Trait 的函数示例
// 这个函数返回一个实现了 Speakable trait 的具体类型
// 编译器会推断出返回类型是 HouseCat
fn create_default_cat() -> impl Speakable {
    HouseCat
}

// 定义一个带有默认实现和向下转换能力的 trait
// 这展示了 trait 的默认方法实现和运行时类型信息
trait DowncastableCreature {
    // 带有默认实现的方法
    fn make_sound(&self) {
        println!("(静默...)")  // 默认行为
    }

    // 必须实现的方法：用于运行时类型转换
    // 返回 &dyn Any，允许运行时类型检查和向下转换
    fn as_any(&self) -> &dyn Any;
}

// 定义 LandTortoise（陆龟）结构体
struct LandTortoise;

// 为 LandTortoise 实现 DowncastableCreature trait
impl DowncastableCreature for LandTortoise {
    // 重写默认的 make_sound 方法（可选）
    // 这里使用默认实现，所以不需要重写

    // 必须实现 as_any 方法
    fn as_any(&self) -> &dyn Any {
        self  // 返回 self 的 Any 引用
    }
}
// 主函数：展示各种 trait 用法的示例
fn main() {
    // ==================== 基本 trait 使用示例 ====================

    // 创建 HouseCat 实例并调用其 make_sound 方法
    let house_cat = HouseCat;
    house_cat.make_sound();

    // 创建 DomesticDog 实例并调用其 make_sound 方法
    let domestic_dog = DomesticDog;
    domestic_dog.make_sound();

    // ==================== trait bound 参数示例 ====================

    // 调用接受 impl Trait 参数的函数
    make_animal_speak_twice(&house_cat);

    // ==================== 返回 impl Trait 示例 ====================

    // 调用返回 impl Trait 的函数
    let default_animal = create_default_cat();
    default_animal.make_sound();

    // ==================== trait 对象（动态分发）示例 ====================

    // 创建 trait 对象的向量
    // Box<dyn Speakable> 是动态分发，运行时决定调用哪个方法
    let speakable_creatures: Vec<Box<dyn Speakable>> = vec![Box::new(house_cat), Box::new(domestic_dog)];

    // 遍历所有动物并让它们发声
    speakable_creatures.iter().for_each(|creature| {
        creature.make_sound();
    });

    // ==================== 向下转换示例 ====================

    // 创建可以向下转换的生物向量
    let downcastable_creatures: Vec<Box<dyn DowncastableCreature>> = vec![Box::new(LandTortoise)];

    // 展示运行时类型检查和向下转换
    for creature in downcastable_creatures {
        creature.make_sound();  // 调用默认的 make_sound 方法

        // 使用 downcast_ref 进行运行时类型转换
        if let Some(_tortoise_instance) = creature.as_any().downcast_ref::<LandTortoise>() {
            println!("我是一只陆龟!");  // 只有当类型匹配时才执行
        }
    }

    // ==================== 运算符重载示例 ====================

    // 创建两个坐标点
    let first_point = CoordinatePoint { x_coordinate: 1.0, y_coordinate: 2.0 };
    let second_point = CoordinatePoint { x_coordinate: 1.0, y_coordinate: 2.0 };

    // 使用重载的 + 运算符
    let sum_point = first_point + second_point;
    println!("{:?}", sum_point);  // 使用 Debug trait 打印结果

    // ==================== 更多 trait 示例 ====================

    // 1. Clone trait 示例
    let first_house_cat = HouseCat;
    let _cloned_house_cat = first_house_cat.clone();  // 现在可以克隆了！
    // let moved_house_cat = first_house_cat;  // 这会移动 first_house_cat，所以注释掉了

    // 2. PartialEq trait 示例
    let cat_instance_1 = HouseCat;
    let cat_instance_2 = HouseCat;
    if cat_instance_1 == cat_instance_2 {
        println!("两只家猫是相同的！");
    }

    // 3. Display trait 示例
    let coordinate_location = CoordinatePoint { x_coordinate: 3.14, y_coordinate: 2.71 };
    println!("点坐标: {}", coordinate_location);  // 使用 Display trait
    println!("调试信息: {:?}", coordinate_location);  // 使用 Debug trait

    // 4. WildBird 类型的示例
    let sparrow_bird = WildBird { bird_species: "麻雀".to_string() };
    sparrow_bird.make_sound();
    println!("鸟类信息: {}", sparrow_bird);  // 使用 Display trait

    // 5. 比较两个坐标点
    let point_a = CoordinatePoint { x_coordinate: 1.0, y_coordinate: 2.0 };
    let point_b = CoordinatePoint { x_coordinate: 1.0, y_coordinate: 2.0 };
    if point_a == point_b {
        println!("两个坐标点相等！");
    }

    // ==================== 高级 Trait 概念演示 ====================
    demonstrate_advanced_traits();
}
