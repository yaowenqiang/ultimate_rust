/*
 * Rust 迭代器 (Iterators) 全面学习示例
 *
 * 本项目展示了 Rust 迭代器系统的强大功能，从基础概念到高级应用，
 * 包含详细的中文注释、实用示例和最佳实践指导。
 *
 * 📚 官方文档链接：
 *
 * 🔰 基础概念
 * 1. Rust Book - 迭代器详解:
 *    https://doc.rust-lang.org/book/ch13-02-iterators-and-closures.html
 *
 * 2. Rust by Example - 迭代器:
 *    https://doc.rust-lang.org/rust-by-example/iterators.html
 *
 * ⚙️ 迭代器 trait
 * 3. Iterator trait 完整文档:
 *    https://doc.rust-lang.org/std/iter/trait.Iterator.html
 *
 * 4. ExactSizeIterator trait:
 *    https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html
 *
 * 5. DoubleEndedIterator trait:
 *    https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html
 *
 * 🔄 迭代器适配器
 * 6. 迭代器适配器方法:
 *    https://doc.rust-lang.org/std/iter/index.html#adapters
 *
 * 7. 迭代器消费者方法:
 *    https://doc.rust-lang.org/std/iter/index.html#consumers
 *
 * 🚀 高级概念
 * 8. 迭代器的性能和零成本抽象:
 *    https://doc.rust-lang.org/book/ch13-04-performance.html
 *
 * 9. 自定义迭代器模式:
 *    https://rust-unofficial.github.io/patterns/patterns/behavioural/iterator.html
 *
 * 10. 并行迭代器 (rayon):
 *     https://docs.rs/rayon/latest/rayon/
 *
 * 🎯 核心学习要点：
 *
 * 🔹 迭代器的本质
 * - 迭代器是处理元素序列的模式
 * - 惰性求值：只在需要时才计算下一个值
 * - 所有权管理：迭代器可以获取、借用或迭代引用
 *
 * 🔹 迭代器的优势
 * - 链式操作：可以组合多个操作
 * - 性能优化：编译器可以优化迭代器链
 * - 内存效率：不需要中间集合
 *
 * 🔹 自定义迭代器
 * - 实现 Iterator trait
 * - 定义关联类型 Item
 * - 实现 next() 方法
 *
 * 🔹 迭代器分类
 * - 适配器：转换迭代器（map, filter 等）
 * - 消费者：消耗迭代器（collect, count 等）
 * - 特殊迭代器：ExactSize, DoubleEnded 等
 */

// ==================== 1. 基础计数器迭代器 ====================

/// 计数器迭代器结构体
///
/// 这是一个简单的自定义迭代器示例，用于生成从 1 到 max 的数字序列。
///
/// # 字段说明
/// * `count` - 当前计数，从 0 开始
/// * `max` - 最大值，迭代器生成的数字不会超过这个值
///
/// # 使用场景
/// - 生成数字序列用于测试
/// - 演示自定义迭代器的基本实现
/// - 作为其他复杂迭代器的基础
///
/// # 文档链接
/// - [自定义迭代器指南](https://doc.rust-lang.org/book/ch13-02-iterators-and-closures.html#creating-our-own-iterators)
/// - [Iterator trait 文档](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
#[derive(Debug, Clone)]
struct Counter {
    /// 当前计数值，内部使用 0-based 索引
    count: u32,
    /// 最大计数值（不包含），即迭代器生成的数字范围是 [1, max]
    max: u32,
}

impl Counter {
    /// 创建新的计数器迭代器
    ///
    /// # 参数
    /// * `max` - 最大值，迭代器将生成从 1 到 max 的数字
    ///
    /// # 返回值
    /// 返回一个 Counter 实例
    ///
    /// # 示例
    /// ```
    /// let counter = Counter::new(5);
    /// // 将生成序列: 1, 2, 3, 4, 5
    /// ```
    ///
    /// # 设计考虑
    /// - 内部使用 0-based 计数 (count)，但生成 1-based 数字
    /// - 这种设计简化了边界条件的处理
    /// - 符合 Rust 的零基索引惯例
    fn new(max: u32) -> Counter {
        Counter {
            count: 0,  // 从 0 开始计数
            max        // 设置最大值
        }
    }

    /// 获取当前计数器的状态信息
    ///
    /// # 返回值
    /// 返回一个元组 (当前值, 剩余数量, 总数)
    fn status(&self) -> (u32, u32, u32) {
        let current = if self.count == 0 { 0 } else { self.count };
        let remaining = self.max - self.count;
        let total = self.max;
        (current, remaining, total)
    }
}

/// 为 Counter 实现 ExactSizeIterator trait
///
/// ExactSizeIterator 表示迭代器能够精确知道剩余元素数量。
/// 这对于某些优化很有用，比如预分配容器大小。
///
/// # 实现要求
/// - 必须提供准确的 len() 方法
/// - len() 应该返回剩余元素的数量
///
/// # 优势
/// - 可以预分配 Vec 等容器的大小
/// - 某些算法可以利用已知大小的信息进行优化
impl ExactSizeIterator for Counter {
    /// 返回迭代器中剩余的元素数量
    ///
    /// # 返回值
    /// 剩余元素的数量，类型为 usize
    ///
    /// # 注意事项
    /// - 返回值是剩余元素，不是总数
    /// - 使用 as 转换时要注意数值溢出
    fn len(&self) -> usize {
        (self.max - self.count) as usize
    }
}

/// 为 Counter 实现 Iterator trait
///
/// 这是自定义迭代器的核心实现。
/// Iterator trait 是 Rust 中迭代器系统的基础。
///
/// # 关联类型
/// - Item: 迭代器产生的元素类型
///
/// # 核心方法
/// - next(): 获取下一个元素，是迭代器的心脏
///
/// # 文档链接
/// - [Iterator trait 详细说明](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
/// - [迭代器设计模式](https://rust-unofficial.github.io/patterns/patterns/behavioural/iterator.html)
impl Iterator for Counter {
    /// 迭代器产生的元素类型
    ///
    /// 这里我们产生 u32 类型的数字序列。
    /// 关联类型让编译器知道迭代器会产生什么类型的值。
    type Item = u32;

    /// 获取迭代器的下一个元素
    ///
    /// 这是迭代器的核心方法，定义了迭代的行为。
    ///
    /// # 返回值语义
    /// * `Some(value)` - 还有元素，返回下一个值
    /// * `None` - 迭代结束，没有更多元素
    ///
    /// # 算法逻辑
    /// 1. 检查是否还有剩余元素 (count < max)
    /// 2. 如果有，递增计数器并返回新值
    /// 3. 如果没有，返回 None 表示迭代结束
    ///
    /// # 状态管理
    /// - count 字段跟踪当前状态
    /// - 每次调用 next() 都会修改内部状态
    /// - 这是迭代器模式的典型实现
    ///
    /// # 边界条件处理
    /// - 正确处理 max = 0 的情况（空迭代器）
    /// - 正确处理 u32 溢出（虽然在实际使用中很少遇到）
    ///
    /// # 文档链接
    /// - [next() 方法文档](https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next)
    /// - [Option 类型详解](https://doc.rust-lang.org/std/option/enum.Option.html)
    fn next(&mut self) -> Option<Self::Item> {
        // 检查是否还有未生成的数字
        if self.count < self.max {
            // 递增计数器（先递增，因为我们要生成 1-based 数字）
            self.count += 1;
            // 返回当前计数值
            Some(self.count)
        } else {
            // 已经达到最大值，迭代结束
            None
        }
    }
}

// ==================== 2. 斐波那契迭代器 ====================

/// 斐波那契数列迭代器
///
/// 生成斐波那契数列：0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...
///
/// # 数学定义
/// F(0) = 0, F(1) = 1
/// F(n) = F(n-1) + F(n-2) for n > 1
///
/// # 字段说明
/// * `current` - 当前斐波那契数
/// * `next` - 下一个斐波那契数
/// * `count` - 已生成的数字数量
///
/// # 文档链接
/// - [斐波那契数列](https://en.wikipedia.org/wiki/Fibonacci_number)
/// - [Rust 迭代器示例](https://doc.rust-lang.org/rust-by-example/trait/iter.html)
#[derive(Debug, Clone)]
struct Fibonacci {
    current: u64,    // 当前的斐波那契数
    next: u64,       // 下一个斐波那契数
    count: usize,    // 已生成的数字数量
}

impl Fibonacci {
    /// 创建新的斐波那契迭代器
    ///
    /// # 返回值
    /// 返回一个从 0 开始的斐波那契迭代器
    fn new() -> Fibonacci {
        Fibonacci {
            current: 0,   // F(0) = 0
            next: 1,      // F(1) = 1
            count: 0,     // 还没有生成任何数字
        }
    }

    /// 创建限制长度的斐波那契迭代器
    ///
    /// # 参数
    /// * `max_count` - 最大生成数量
    fn with_limit(max_count: usize) -> LimitedFibonacci {
        LimitedFibonacci {
            fibonacci: Fibonacci::new(),
            remaining: max_count,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // 检查是否会溢出
        if self.current > u64::MAX / 2 && self.next > u64::MAX / 2 {
            return None; // 防止溢出
        }

        let result = self.current;
        let new_next = self.current + self.next;

        self.current = self.next;
        self.next = new_next;
        self.count += 1;

        Some(result)
    }
}

/// 限制长度的斐波那契迭代器
///
/// 包装 Fibonacci 迭代器，限制生成的数量
#[derive(Debug)]
struct LimitedFibonacci {
    fibonacci: Fibonacci,
    remaining: usize,
}

impl Iterator for LimitedFibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            self.fibonacci.next()
        }
    }
}

// ==================== 3. 素数迭代器 ====================

/// 素数迭代器
///
/// 生成无限素数序列：2, 3, 5, 7, 11, 13, 17, 19, 23, 29, ...
///
/// # 算法说明
/// 使用简单的试除法检查每个数字是否为素数。
/// 虽然不是最高效的算法，但易于理解和实现。
///
/// # 字段说明
/// * `current` - 当前检查的数字
/// * `primes` - 已找到的素数列表，用于优化检查
///
/// # 性能考虑
/// - 随着素数数量增加，检查速度会变慢
/// - 适合教学和演示，生产环境建议使用更高效的算法
///
/// # 文档链接
/// - [素数算法](https://en.wikipedia.org/wiki/Prime_number)
/// - [埃拉托斯特尼筛法](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes)
#[derive(Debug)]
struct PrimeIterator {
    current: u64,
    primes: Vec<u64>,
}

impl PrimeIterator {
    /// 创建新的素数迭代器
    fn new() -> PrimeIterator {
        PrimeIterator {
            current: 1, // 从 1 开始，第一个检查 2
            primes: Vec::new(),
        }
    }

    /// 检查一个数字是否为素数
    ///
    /// # 参数
    /// * `n` - 要检查的数字
    ///
    /// # 返回值
    /// true 如果是素数，false 否则
    ///
    /// # 算法
    /// 试除法：检查从 2 到 √n 的所有已知素数
    fn is_prime(&self, n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }

        // 只检查到平方根
        let limit = (n as f64).sqrt() as u64 + 1;
        for &prime in &self.primes {
            if prime > limit {
                break;
            }
            if n % prime == 0 {
                return false;
            }
        }
        true
    }
}

impl Iterator for PrimeIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.current += 1;

            if self.is_prime(self.current) {
                self.primes.push(self.current);
                return Some(self.current);
            }

            // 防止无限循环（理论上不会到达，但作为安全措施）
            if self.current > u64::MAX - 1 {
                return None;
            }
        }
    }
}

// ==================== 主函数演示 ====================

fn main() {
    println!("=== Rust 迭代器全面学习示例 ===\n");

    // ==================== 基础计数器迭代器演示 ====================
    println!("1. 基础计数器迭代器演示:");

    // 创建一个计数器，生成 1-10 的数字
    let counter = Counter::new(10);
    println!("创建计数器，状态: {:?}", counter.status());

    // 收集所有数字到向量
    let numbers: Vec<u32> = counter.collect();
    println!("收集的数字: {:?}", numbers);

    // 演示迭代器的各种操作
    println!("\n🔢 计数器迭代器操作演示:");

    // 重新创建计数器（因为之前的已经被消耗）
    let counter2 = Counter::new(5);

    // 使用 map 进行转换
    let doubled: Vec<u32> = counter2.map(|x| x * 2).collect();
    println!("原数字 1-5 乘以 2: {:?}", doubled);

    // 使用 filter 进行过滤
    let counter3 = Counter::new(10);
    let evens: Vec<u32> = counter3.filter(|&x| x % 2 == 0).collect();
    println!("1-10 中的偶数: {:?}", evens);

    // 演示 ExactSizeIterator
    let counter4 = Counter::new(7);
    println!("新建计数器长度: {}", counter4.len());

    // 消费一些元素后再次检查长度
    let mut counter5 = Counter::new(7);
    println!("消耗第一个元素前长度: {}", counter5.len());
    counter5.next();
    println!("消耗第一个元素后长度: {}", counter5.len());

    // ==================== 斐波那契迭代器演示 ====================
    println!("\n2. 斐波那契迭代器演示:");

    // 生成前 10 个斐波那契数
    let fib_numbers: Vec<u64> = Fibonacci::with_limit(10).collect();
    println!("前 10 个斐波那契数: {:?}", fib_numbers);

    // 演示斐波那契数列的数学性质
    println!("\n🔢 斐波那契数列性质演示:");
    let fib_iter = Fibonacci::with_limit(15);
    let fib_vec: Vec<u64> = fib_iter.collect();

    // 验证黄金比例（相邻两项的比值趋近于黄金比例）
    if fib_vec.len() >= 2 {
        let golden_ratio = (fib_vec[fib_vec.len()-1] as f64) / (fib_vec[fib_vec.len()-2] as f64);
        println!("第14/13项比值: {:.6} (黄金比例 ≈ 1.618)", golden_ratio);
    }

    // ==================== 素数迭代器演示 ====================
    println!("\n3. 素数迭代器演示:");

    // 生成前 10 个素数
    let prime_numbers: Vec<u64> = PrimeIterator::new().take(10).collect();
    println!("前 10 个素数: {:?}", prime_numbers);

    // 演示素数的性质
    println!("\n🔢 素数性质演示:");

    // 生成前 20 个素数进行统计分析
    let primes_20: Vec<u64> = PrimeIterator::new().take(20).collect();
    println!("前 20 个素数: {:?}", primes_20);

    // 计算素数间距
    let mut gaps = Vec::new();
    for i in 1..primes_20.len() {
        gaps.push(primes_20[i] - primes_20[i-1]);
    }
    println!("素数间距: {:?}", gaps);

    // 找出最大和最小间距
    let max_gap = gaps.iter().max().unwrap_or(&0);
    let min_gap = gaps.iter().min().unwrap_or(&0);
    println!("最大间距: {}, 最小间距: {}", max_gap, min_gap);

    // ==================== 迭代器链式操作演示 ====================
    println!("\n4. 迭代器链式操作演示:");

    // 复杂的迭代器链操作
    let result: Vec<String> = Counter::new(20)
        .filter(|&x| x % 2 == 1)           // 只保留奇数
        .map(|x| x * x)                    // 计算平方
        .filter(|&x| x > 10)               // 大于 10
        .map(|x| format!("平方: {}", x))    // 转换为字符串
        .take(5)                           // 只取前 5 个
        .collect();

    println!("1-20中奇数平方>10的前5个: {:?}", result);

    // 演示惰性求值
    println!("\n⚡ 惰性求值演示:");
    let lazy_chain = Counter::new(1000000)
        .map(|x| {
            // 模拟昂贵的计算
            if x <= 3 {
                println!("  正在计算第 {} 项...", x);
            }
            x * x
        })
        .filter(|&x| x > 100);

    println!("创建了惰性迭代器链，但尚未执行计算");

    // 现在执行前几个计算
    let first_few: Vec<u32> = lazy_chain.take(3).collect();
    println!("前几个结果: {:?}", first_few);

    // ==================== 性能对比演示 ====================
    println!("\n5. 性能对比演示:");

    use std::time::Instant;

    let n = 100000;

    // 传统 for 循环方式
    let start = Instant::now();
    let mut sum_for = 0u64;
    for i in 1..=n {
        sum_for += i * i;
    }
    let duration_for = start.elapsed();

    // 迭代器方式
    let start = Instant::now();
    let sum_iter: u64 = (1..=n).map(|x| x * x).sum();
    let duration_iter = start.elapsed();

    println!("传统 for 循环: 和 = {}, 耗时 = {:?}", sum_for, duration_for);
    println!("迭代器方式:   和 = {}, 耗时 = {:?}", sum_iter, duration_iter);

    if sum_for == sum_iter {
        println!("✅ 结果一致！");
        if duration_iter < duration_for {
            println!("🚀 迭代器更快！");
        } else {
            println!("📊 传统循环更快（在小数据量上可能如此）");
        }
    }

    // ==================== 高级迭代器特性演示 ====================
    println!("\n6. 高级迭代器特性演示:");

    // DoubleEndedIterator 演示
    let range_vec: Vec<i32> = (1..=10).collect();
    println!("原始向量: {:?}", range_vec);

    // 从两端同时迭代
    let mut range_iter = range_vec.iter();
    println!("从头部: {:?}", range_iter.next());
    println!("从尾部: {:?}", range_iter.next_back());
    println!("从头部: {:?}", range_iter.next());
    println!("从尾部: {:?}", range_iter.next_back());

    // enumerate() 演示
    println!("\n🔢 enumerate() 演示:");
    let enumerated: Vec<(usize, i32)> = (10..=15).enumerate().map(|(i, x)| (i, x)).collect();
    println!("带索引的枚举: {:?}", enumerated);

    println!("\n=== 迭代器学习完成 ===");
    println!("迭代器是 Rust 中强大且高效的工具！");
    println!("它们提供了链式操作、惰性求值和零成本抽象。");
}
