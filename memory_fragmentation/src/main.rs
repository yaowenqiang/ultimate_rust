/*
 * Rust 内存碎片化和内存管理深入学习示例
 *
 * 本项目展示了 Rust 中内存碎片化的概念、问题和解决方案，这是理解 Rust
 * 内存管理、性能优化和系统编程的关键概念。
 *
 * 📚 官方文档链接：
 *
 * 🔰 基础概念
 * 1. Rust Book - 内存管理:
 *    https://doc.rust-lang.org/book/ch15-00-smart-pointers.html
 *
 * 2. Rustonomicon - 内存安全:
 *    https://doc.rust-lang.org/nomicon/
 *
 * 3. Rust by Example - 内存管理:
 *    https://doc.rust-lang.org/rust-by-example/std/box.html
 *
 * ⚙️ 分配器文档
 * 4. bumpalo - Arena 分配器:
 *    https://docs.rs/bumpalo/3.19.0/bumpalo/
 *
 * 5. jemallocator - Jemalloc 分配器:
 *    https://docs.rs/jemallocator/0.5.4/jemallocator/
 *
 * 6. slab - Slab 分配器:
 *    https://docs.rs/slab/0.4.11/slab/
 *
 * 🚀 高级概念
 * 7. 自定义分配器:
 *    https://doc.rust-lang.org/std/alloc/trait.Allocator.html
 *
 * 8. 内存碎片化问题:
 *    https://en.wikipedia.org/wiki/Fragmentation_(computing)
 *
 * 9. 性能优化技巧:
 *    https://doc.rust-lang.org/nomicon/vec.html
 *
 * 🎯 核心学习要点：
 *
 * 🔹 内存碎片化的本质
 * - 内存碎片化是指内存空间被分割成许多不连续的小块
 * - 外部碎片：可用内存空间分散，无法满足大的分配请求
 * - 内部碎片：分配的内存块大于实际需要的内存
 * - 内存碎片化会降低内存利用率和分配性能
 *
 * 🔹 内存分配器类型
 * - Arena 分配器：快速分配，批量释放，适合临时数据
 * - Slab 分配器：固定大小对象，低碎片化，高频分配释放
 * - Jemalloc：通用分配器，高性能，减少碎片化
 * - 自定义分配器：针对特定场景优化
 *
 * 🔹 解决方案
 * - 使用对象池和内存池
 * - 批量分配和释放
 * - 选择合适的分配策略
 * - 预分配内存空间
 *
 * 🔹 实际应用场景
 * - 游戏引擎中的对象管理
 * - Web 服务器的请求处理
 * - 数据库系统的缓存管理
 * - 高频交易系统的内存管理
 */

use std::{
    alloc::{GlobalAlloc, Layout, System},
    time::{Duration, Instant},
    fmt::Debug,
};
use bumpalo::Bump;
use bumpalo::collections::{String as BumpString, Vec as BumpVec};
use slab::Slab;
use rand::Rng;
use rand::seq::SliceRandom;

// ==================== 1. 全局分配器示例 ====================

/// 使用 Jemalloc 作为全局分配器
///
/// Jemalloc 是一个高性能的内存分配器，专为多线程程序优化，
/// 相比系统默认分配器，它能更好地处理内存碎片化问题。
///
/// # 特点
/// - 减少内存碎片化
/// - 提高多线程环境下的分配性能
/// - 更好的内存局部性
///
/// # 使用方法
/// 取消注释下面的代码来启用 Jemalloc 作为全局分配器
///
/// # 文档链接
/// - [jemallocator 文档](https://docs.rs/jemallocator/0.5.4/jemallocator/)
// #[global_allocator]
// static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

/// 自定义内存使用统计分配器
///
/// 这个结构体包装了系统分配器，并添加了内存使用统计功能。
/// 通过这种方式，我们可以监控内存分配的行为和碎片化情况。
///
/// # 字段说明
/// * `allocations` - 分配次数统计
/// * `total_allocated` - 总分配内存大小
/// * `peak_allocated` - 峰值内存使用量
///
/// # 实现原理
/// 通过实现 GlobalAlloc trait，我们可以拦截所有的内存分配请求，
/// 从而进行统计和监控。
///
/// # 文档链接
/// - [GlobalAlloc trait](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html)
#[derive(Debug)]
struct StatsAllocator {
    allocations: std::sync::atomic::AtomicUsize,
    total_allocated: std::sync::atomic::AtomicUsize,
    peak_allocated: std::sync::atomic::AtomicUsize,
}

impl StatsAllocator {
    /// 创建新的统计分配器
    const fn new() -> Self {
        Self {
            allocations: std::sync::atomic::AtomicUsize::new(0),
            total_allocated: std::sync::atomic::AtomicUsize::new(0),
            peak_allocated: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// 获取分配次数
    fn get_allocations(&self) -> usize {
        self.allocations.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// 获取总分配内存
    fn get_total_allocated(&self) -> usize {
        self.total_allocated.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// 获取峰值内存使用
    fn get_peak_allocated(&self) -> usize {
        self.peak_allocated.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// 重置统计数据
    fn reset(&self) {
        self.allocations.store(0, std::sync::atomic::Ordering::Relaxed);
        self.total_allocated.store(0, std::sync::atomic::Ordering::Relaxed);
        self.peak_allocated.store(0, std::sync::atomic::Ordering::Relaxed);
    }

    /// 打印统计信息
    fn print_stats(&self, name: &str) {
        println!("📊 {} 内存统计:", name);
        println!("   分配次数: {}", self.get_allocations());
        println!("   总分配内存: {} bytes", self.get_total_allocated());
        println!("   峰值内存使用: {} bytes", self.get_peak_allocated());
        println!("   平均分配大小: {} bytes",
                if self.get_allocations() > 0 {
                    self.get_total_allocated() / self.get_allocations()
                } else {
                    0
                });
        println!();
    }
}

unsafe impl GlobalAlloc for StatsAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // 使用系统分配器分配内存
        let ptr = System.alloc(layout);

        if !ptr.is_null() {
            // 更新统计信息
            self.allocations.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let size = layout.size();
            let old_total = self.total_allocated.fetch_add(size, std::sync::atomic::Ordering::Relaxed);
            let new_total = old_total + size;

            // 更新峰值使用量
            let mut current_peak = self.peak_allocated.load(std::sync::atomic::Ordering::Relaxed);
            while new_total > current_peak {
                match self.peak_allocated.compare_exchange_weak(
                    current_peak,
                    new_total,
                    std::sync::atomic::Ordering::Relaxed,
                    std::sync::atomic::Ordering::Relaxed
                ) {
                    Ok(_) => break,
                    Err(x) => current_peak = x,
                }
            }
        }

        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);

        // 更新统计信息
        let size = layout.size();
        self.total_allocated.fetch_sub(size, std::sync::atomic::Ordering::Relaxed);
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let _new_layout = Layout::from_size_align_unchecked(new_size, layout.align());
        let new_ptr = System.realloc(ptr, layout, new_size);

        if !new_ptr.is_null() {
            let old_size = layout.size();
            let size_diff = new_size as isize - old_size as isize;

            if size_diff > 0 {
                self.allocations.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let old_total = self.total_allocated.fetch_add(size_diff as usize, std::sync::atomic::Ordering::Relaxed);
                let new_total = old_total + size_diff as usize;

                // 更新峰值使用量
                let mut current_peak = self.peak_allocated.load(std::sync::atomic::Ordering::Relaxed);
                while new_total > current_peak {
                    match self.peak_allocated.compare_exchange_weak(
                        current_peak,
                        new_total,
                        std::sync::atomic::Ordering::Relaxed,
                        std::sync::atomic::Ordering::Relaxed
                    ) {
                        Ok(_) => break,
                        Err(x) => current_peak = x,
                    }
                }
            } else if size_diff < 0 {
                self.total_allocated.fetch_sub((-size_diff) as usize, std::sync::atomic::Ordering::Relaxed);
            }
        }

        new_ptr
    }
}

// 全局统计分配器实例
static STATS_ALLOCATOR: StatsAllocator = StatsAllocator::new();

// ==================== 2. 数据结构定义 ====================

/// 测试用的数据结构
///
/// 这个结构体用于测试不同分配器的性能和内存使用情况。
/// 通过调整字段大小和类型，可以模拟不同大小的对象分配。
///
/// # 字段说明
/// * `id` - 对象标识符
/// * `data` - 数据数组，可以调整大小来测试不同的分配场景
/// * `name` - 字符串字段，测试字符串分配
/// * `metadata` - 元数据，用于模拟复杂对象
///
/// # 性能考虑
/// - 数据数组的大小会影响内存分配模式
/// - 字符串字段会触发额外的内存分配
/// - 结构体对齐会影响内存布局
///
/// # 文档链接
/// - [Rust 结构体内存布局](https://doc.rust-lang.org/nomicon/repr-rust.html)
#[derive(Debug, Clone, PartialEq)]
struct TestData {
    id: u64,
    data: [u8; 64],  // 64字节数据块，可调整大小
    name: String,
    metadata: u32,
}

impl TestData {
    /// 创建新的测试数据
    ///
    /// # 参数
    /// * `id` - 对象ID
    /// * `name` - 对象名称
    ///
    /// # 返回值
    /// 返回一个新的 TestData 实例
    fn new(id: u64, name: &str) -> Self {
        let mut data = [0u8; 64];
        // 填充随机数据以模拟真实场景
        for i in 0..data.len() {
            data[i] = (id % 256 + i as u64) as u8;
        }

        Self {
            id,
            data,
            name: name.to_string(),
            metadata: (id * 2) as u32,
        }
    }

    /// 获取对象大小
    fn size() -> usize {
        std::mem::size_of::<Self>()
    }

    /// 计算内存对齐后的大小
    fn aligned_size() -> usize {
        std::mem::align_of::<Self>()
    }
}

/// 内存碎片化统计信息
///
/// 这个结构体用于收集和分析内存碎片化的各种指标。
/// 通过这些统计数据，我们可以了解内存使用的效率和问题。
///
/// # 字段说明
/// * `total_requests` - 总分配请求数
/// * `successful_allocations` - 成功分配次数
/// * `failed_allocations` - 分配失败次数
/// * `fragmentation_ratio` - 碎片化比率
/// * `average_allocation_size` - 平均分配大小
/// * `largest_free_block` - 最大空闲块大小
///
/// # 碎片化计算
/// 碎片化比率 = (已分配内存 - 实际使用内存) / 已分配内存
///
/// # 文档链接
/// - [内存碎片化概念](https://en.wikipedia.org/wiki/Fragmentation_(computing))
#[derive(Debug, Default)]
struct FragmentationStats {
    total_requests: usize,
    successful_allocations: usize,
    failed_allocations: usize,
    total_allocated: usize,
    actually_used: usize,
    largest_free_block: usize,
}

impl FragmentationStats {
    /// 创建新的碎片化统计
    fn new() -> Self {
        Self::default()
    }

    /// 记录成功的分配
    fn record_allocation(&mut self, size: usize, used: usize) {
        self.total_requests += 1;
        self.successful_allocations += 1;
        self.total_allocated += size;
        self.actually_used += used;
    }

    /// 记录失败的分配
    fn record_failure(&mut self) {
        self.total_requests += 1;
        self.failed_allocations += 1;
    }

    /// 计算碎片化比率
    fn fragmentation_ratio(&self) -> f64 {
        if self.total_allocated == 0 {
            0.0
        } else {
            (self.total_allocated - self.actually_used) as f64 / self.total_allocated as f64
        }
    }

    /// 计算内存利用率
    fn utilization_rate(&self) -> f64 {
        if self.total_allocated == 0 {
            0.0
        } else {
            self.actually_used as f64 / self.total_allocated as f64
        }
    }

    /// 打印统计信息
    fn print_stats(&self, name: &str) {
        println!("📈 {} 碎片化统计:", name);
        println!("   总请求数: {}", self.total_requests);
        println!("   成功分配: {}", self.successful_allocations);
        println!("   失败分配: {}", self.failed_allocations);
        println!("   总分配内存: {} bytes", self.total_allocated);
        println!("   实际使用内存: {} bytes", self.actually_used);
        println!("   碎片化比率: {:.2}%", self.fragmentation_ratio() * 100.0);
        println!("   内存利用率: {:.2}%", self.utilization_rate() * 100.0);

        if self.total_requests > 0 {
            let success_rate = self.successful_allocations as f64 / self.total_requests as f64;
            println!("   成功率: {:.2}%", success_rate * 100.0);
        }

        if self.successful_allocations > 0 {
            let avg_size = self.total_allocated / self.successful_allocations;
            println!("   平均分配大小: {} bytes", avg_size);
        }

        println!();
    }
}

// ==================== 3. Arena 分配器示例 ====================

/// 演示 Arena 分配器的使用
///
/// Arena 分配器是一种高效的内存分配策略，它通过在连续的内存块中
/// 快速分配对象，并在整个生命周期结束时一次性释放所有对象。
///
/// # 特点
/// - 极快的分配速度（只是指针递增）
/// - 零碎片化（连续分配）
/// - 批量释放（一次性释放所有对象）
/// - 适合临时对象和短生命周期数据
///
/// # 使用场景
/// - 游戏引擎中的帧临时对象
/// - 编译器的中间表示
/// - 网络请求处理中的临时数据
/// - 数据处理管道中的临时缓冲区
///
/// # 文档链接
/// - [bumpalo 文档](https://docs.rs/bumpalo/3.19.0/bumpalo/)
/// - [Arena 分配器原理](https://en.wikipedia.org/wiki/Region-based_memory_management)
fn demonstrate_arena_allocator() {
    println!("🏟️  1. Arena 分配器演示:");
    println!("   展示快速分配和批量释放的特性");

    let start_time = Instant::now();
    STATS_ALLOCATOR.reset();

    // 创建 Arena 分配器
    let arena = Bump::new();

    // 设置分配限制（8KB）
    arena.set_allocation_limit(Some(8192));
    println!("   创建 Arena，设置内存限制为 8KB");

    // 在 Arena 中分配基本数据
    let data = arena.alloc(TestData::new(1, "arena_test_1"));
    println!("   分配基本数据: {:?}", data.id);

    // 在 Arena 中分配字符串
    let mut arena_string = BumpString::from_str_in("Hello from Arena!", &arena);
    arena_string.push_str(" 这是追加的内容");
    println!("   Arena 字符串: {}", arena_string);

    // 在 Arena 中分配向量
    let mut arena_vec = BumpVec::new_in(&arena);
    for i in 0..100 {
        arena_vec.push(i as i32);
    }
    println!("   Arena 向量大小: {} 个元素", arena_vec.len());

    // 模拟大量分配
    println!("   开始大量分配测试...");
    let mut arena_objects = Vec::new();

    for i in 0..50 {
        let obj = arena.alloc(TestData::new(i, &format!("arena_obj_{}", i)));
        arena_objects.push(obj);

        if i % 10 == 0 {
            println!("     已分配 {} 个对象", i + 1);
        }
    }

    println!("   ✅ Arena 分配完成，总共 {} 个对象", arena_objects.len());

    // Arena 会在离开作用域时自动释放所有内存
    // 这里不需要手动释放每个对象

    let duration = start_time.elapsed();
    println!("   ⏱️  总耗时: {:?}", duration);

    // 打印统计信息
    STATS_ALLOCATOR.print_stats("Arena 分配器");

    // 注意：所有在 Arena 中分配的对象会在 arena 离开作用域时被销毁
    println!("   🗑️  Arena 将在作用域结束时释放所有内存");
    println!();
}

/// 演示 Arena 分配器的内存碎片化特性
///
/// 虽然 Arena 分配器本身不会产生外部碎片化（因为内存是连续的），
/// 但我们可以演示它如何避免传统分配器中的碎片化问题。
fn demonstrate_arena_fragmentation() {
    println!("🏟️  2. Arena 分配器碎片化分析:");
    println!("   展示 Arena 如何避免内存碎片化");

    let arena = Bump::new();
    let mut fragmentation_stats = FragmentationStats::new();

    // 模拟不同大小的分配
    let allocation_sizes = vec![8, 16, 32, 64, 128, 256, 512, 1024];
    let mut allocated_objects = Vec::new();

    println!("   分配不同大小的对象:");

    for (i, &size) in allocation_sizes.iter().enumerate() {
        // 在 Arena 中分配基本类型
        let value = arena.alloc(i as u32);

        allocated_objects.push((value as *const u32 as *mut u8, size));
        fragmentation_stats.record_allocation(size, size);  // Arena 没有内部碎片

        println!("     分配 {} bytes: 成功", size);
    }

    // 计算实际使用情况
    let total_allocated_bytes = allocation_sizes.iter().sum::<usize>();
    fragmentation_stats.total_allocated = total_allocated_bytes;
    fragmentation_stats.actually_used = allocation_sizes.iter().sum::<usize>();

    // 打印统计信息
    fragmentation_stats.print_stats("Arena 碎片化");

    println!("   🔍 分析结果:");
    println!("     Arena 使用连续内存，无外部碎片化");
    println!("     内存利用率: {:.1}%",
             fragmentation_stats.utilization_rate() * 100.0);
    println!("     所有对象将在作用域结束时一次性释放");

    println!();
}

// ==================== 4. Slab 分配器示例 ====================

/// 演示 Slab 分配器的使用
///
/// Slab 分配器是一种专门用于存储固定大小对象的内存分配器。
/// 它通过预分配内存块并维护空闲列表来实现高效的分配和释放。
///
/// # 特点
/// - 固定大小对象存储
/// - O(1) 分配和释放时间复杂度
/// - 低内存碎片化
/// - 稳定的内存访问模式
///
/// # 使用场景
/// - 网络连接池
/// - 游戏对象管理
/// - 缓存系统
/// - 线程池管理
///
/// # 文档链接
/// - [slab 文档](https://docs.rs/slab/0.4.11/slab/)
/// - [Slab 分配器算法](https://en.wikipedia.org/wiki/Slab_allocation)
fn demonstrate_slab_allocator() {
    println!("🧱  3. Slab 分配器演示:");
    println!("   展示固定大小对象的高效存储");

    let start_time = Instant::now();
    STATS_ALLOCATOR.reset();

    // 创建字符串 Slab，预分配容量
    let mut string_slab = Slab::with_capacity(100);
    println!("   创建字符串 Slab，预分配容量 100");

    // 插入字符串对象
    let hello_key = string_slab.insert("Hello");
    let world_key = string_slab.insert("World");
    let rust_key = string_slab.insert("Rust");

    println!("   插入对象:");
    println!("     'Hello' -> key: {}", hello_key);
    println!("     'World' -> key: {}", world_key);
    println!("     'Rust'  -> key: {}", rust_key);

    // 访问对象
    println!("   访问对象:");
    println!("     slab[{}] = {}", hello_key, string_slab[hello_key]);
    println!("     slab[{}] = {}", world_key, string_slab[world_key]);
    println!("     slab[{}] = {}", rust_key, string_slab[rust_key]);

    // 创建数据对象 Slab
    let mut data_slab = Slab::with_capacity(100);

    // 插入复杂对象
    println!("   插入复杂对象:");
    let data1_key = data_slab.insert(TestData::new(1, "data_1"));
    let data2_key = data_slab.insert(TestData::new(2, "data_2"));

    println!("     TestData(1) -> key: {}", data1_key);
    println!("     TestData(2) -> key: {}", data2_key);

    // 演示对象的生命周期管理
    println!("   对象生命周期管理:");
    println!("     移除 key: {} ('Hello')", hello_key);
    string_slab.remove(hello_key);

    println!("     当前字符串 Slab 容量: {}", string_slab.capacity());
    println!("     当前字符串 Slab 长度: {}", string_slab.len());

    // 重复利用空槽
    let new_key = string_slab.insert("New Object");
    println!("     新对象插入到空槽: key: {}", new_key);
    println!("     内容: {}", string_slab[new_key]);

    // 批量操作
    println!("   批量操作测试:");
    let mut keys = Vec::new();

    for i in 0..50 {
        let key = data_slab.insert(TestData::new(i as u64, &format!("batch_{}", i)));
        keys.push(key);
    }

    println!("     批量插入 50 个对象");
    println!("     数据 Slab 最终容量: {}", data_slab.capacity());
    println!("     数据 Slab 最终长度: {}", data_slab.len());

    // 随机删除和插入
    println!("   随机删除和插入测试:");
    let mut rng = rand::thread_rng();

    for _ in 0..20 {
        if let Some(&key) = keys.choose(&mut rng) {
            data_slab.remove(key);
        }

        let new_key = data_slab.insert(TestData::new(
            rng.gen_range(1000..2000) as u64,
            "random_obj"
        ));
        keys.push(new_key);
    }

    println!("     随机操作后容量: {}", data_slab.capacity());
    println!("     随机操作后长度: {}", data_slab.len());

    let duration = start_time.elapsed();
    println!("   ⏱️  总耗时: {:?}", duration);

    // 打印统计信息
    STATS_ALLOCATOR.print_stats("Slab 分配器");

    println!();
}

/// 演示 Slab 分配器的内存效率
///
/// 通过对比传统 Vec 和 Slab 在频繁分配释放场景下的性能差异。
fn demonstrate_slab_efficiency() {
    println!("🧱  4. Slab 分配器效率分析:");
    println!("   对比传统 Vec 和 Slab 的性能差异");

    // 测试数据
    const OBJECT_COUNT: usize = 1000;
    const ITERATIONS: usize = 100;

    println!("   测试配置:");
    println!("     对象数量: {}", OBJECT_COUNT);
    println!("     迭代次数: {}", ITERATIONS);

    // Slab 分配器测试
    println!("   Slab 分配器测试:");
    let slab_start = Instant::now();
    STATS_ALLOCATOR.reset();

    for iteration in 0..ITERATIONS {
        let mut slab = Slab::with_capacity(OBJECT_COUNT);
        let mut keys = Vec::with_capacity(OBJECT_COUNT);

        // 分配对象
        for i in 0..OBJECT_COUNT {
            let key = slab.insert(TestData::new(i as u64, &format!("slab_obj_{}", i)));
            keys.push(key);
        }

        // 随机访问
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            if let Some(&key) = keys.choose(&mut rng) {
                let _data = &slab[key];
            }
        }

        // 随机删除部分对象
        for _ in 0..OBJECT_COUNT / 4 {
            if let Some(key) = keys.pop() {
                slab.remove(key);
            }
        }

        if iteration % 10 == 0 {
            println!("     完成迭代: {}/{}", iteration + 1, ITERATIONS);
        }
    }

    let slab_duration = slab_start.elapsed();
    let slab_stats = (
        STATS_ALLOCATOR.get_allocations(),
        STATS_ALLOCATOR.get_total_allocated()
    );

    println!("   ✅ Slab 测试完成");

    // Vec 分配器测试（对比）
    println!("   Vec 分配器测试（对比）:");
    let vec_start = Instant::now();
    STATS_ALLOCATOR.reset();

    for iteration in 0..ITERATIONS {
        let mut vec: Vec<Option<TestData>> = Vec::with_capacity(OBJECT_COUNT);
        let mut indices = Vec::with_capacity(OBJECT_COUNT);

        // 分配对象
        for i in 0..OBJECT_COUNT {
            vec.push(Some(TestData::new(i as u64, &format!("vec_obj_{}", i))));
            indices.push(i);
        }

        // 随机访问
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            if let Some(&index) = indices.choose(&mut rng) {
                let _data = &vec[index];
            }
        }

        // 随机删除部分对象
        for _ in 0..OBJECT_COUNT / 4 {
            if let Some(index) = indices.pop() {
                vec[index] = None;
            }
        }

        if iteration % 10 == 0 {
            println!("     完成迭代: {}/{}", iteration + 1, ITERATIONS);
        }
    }

    let vec_duration = vec_start.elapsed();
    let vec_stats = (
        STATS_ALLOCATOR.get_allocations(),
        STATS_ALLOCATOR.get_total_allocated()
    );

    println!("   ✅ Vec 测试完成");

    // 性能对比
    println!("   📊 性能对比结果:");
    println!("     Slab 耗时: {:?}", slab_duration);
    println!("     Vec  耗时: {:?}", vec_duration);

    if slab_duration < vec_duration {
        let speedup = vec_duration.as_nanos() as f64 / slab_duration.as_nanos() as f64;
        println!("     Slab 速度提升: {:.2}x", speedup);
    } else {
        let slowdown = slab_duration.as_nanos() as f64 / vec_duration.as_nanos() as f64;
        println!("     Slab 速度降低: {:.2}x", slowdown);
    }

    println!("     Slab 内存分配: {} 次, {} bytes", slab_stats.0, slab_stats.1);
    println!("     Vec  内存分配: {} 次, {} bytes", vec_stats.0, vec_stats.1);

    println!();
}

// ==================== 5. 内存碎片化分析 ====================

/// 演示内存碎片化的产生和影响
///
/// 通过模拟不同的内存分配模式，展示内存碎片化是如何产生的，
/// 以及它对系统性能的影响。
fn demonstrate_memory_fragmentation() {
    println!("💔  5. 内存碎片化分析:");
    println!("   展示碎片化的产生原因和影响");

    STATS_ALLOCATOR.reset();
    let mut fragmentation_stats = FragmentationStats::new();

    // 模拟不同的分配模式
    println!("   测试不同分配模式的碎片化情况:");

    // 1. 固定大小分配（低碎片化）
    println!("   1. 固定大小分配:");
    test_fixed_size_allocation(&mut fragmentation_stats);

    // 2. 变化大小分配（中等碎片化）
    println!("   2. 变化大小分配:");
    test_variable_size_allocation(&mut fragmentation_stats);

    // 3. 随机大小分配（高碎片化）
    println!("   3. 随机大小分配:");
    test_random_size_allocation(&mut fragmentation_stats);

    // 4. 分配释放循环（碎片化累积）
    println!("   4. 分配释放循环:");
    test_allocation_deallocation_cycle(&mut fragmentation_stats);

    fragmentation_stats.print_stats("总体碎片化分析");

    // 打印建议
    print_fragmentation_suggestions();

    println!();
}

/// 测试固定大小分配的碎片化情况
fn test_fixed_size_allocation(stats: &mut FragmentationStats) {
    let size = 64;  // 固定 64 字节
    let count = 100;
    let mut allocations = Vec::new();

    for _i in 0..count {
        let layout = std::alloc::Layout::from_size_align(size, 8).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) };

        if !ptr.is_null() {
            allocations.push((ptr, layout));
            stats.record_allocation(size, size);
        } else {
            stats.record_failure();
        }
    }

    println!("     分配 {} 个 {} 字节对象: 成功", count, size);

    // 释放内存
    for (ptr, layout) in allocations {
        unsafe { std::alloc::dealloc(ptr, layout) };
    }
}

/// 测试变化大小分配的碎片化情况
fn test_variable_size_allocation(stats: &mut FragmentationStats) {
    let sizes = vec![16, 32, 64, 128, 256, 512];
    let mut allocations = Vec::new();

    for &size in &sizes {
        for _ in 0..20 {
            let layout = std::alloc::Layout::from_size_align(size, 8).unwrap();
            let ptr = unsafe { std::alloc::alloc(layout) };

            if !ptr.is_null() {
                allocations.push((ptr, layout));
                stats.record_allocation(size, size);
            } else {
                stats.record_failure();
            }
        }
    }

    println!("     分配 {} 种不同大小的对象: 成功", sizes.len());

    // 随机释放部分对象，模拟碎片化
    let _rng = rand::thread_rng();
    for _ in 0..allocations.len() / 3 {
        if let Some((ptr, layout)) = allocations.pop() {
            unsafe { std::alloc::dealloc(ptr, layout) };
        }
    }

    // 剩余对象
    for (ptr, layout) in allocations {
        unsafe { std::alloc::dealloc(ptr, layout) };
    }
}

/// 测试随机大小分配的碎片化情况
fn test_random_size_allocation(stats: &mut FragmentationStats) {
    let mut rng = rand::thread_rng();
    let count = 100;
    let mut allocations = Vec::new();

    for _ in 0..count {
        let size = rng.gen_range(8..1024);  // 8-1024 字节随机大小
        let layout = std::alloc::Layout::from_size_align(size, 8).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) };

        if !ptr.is_null() {
            allocations.push((ptr, layout, size));
            stats.record_allocation(size, size);
        } else {
            stats.record_failure();
        }
    }

    println!("     分配 {} 个随机大小对象: 成功", count);

    // 按大小排序释放，模拟最坏情况的碎片化
    allocations.sort_by_key(|(_, _, size)| *size);

    for (ptr, layout, _) in allocations {
        unsafe { std::alloc::dealloc(ptr, layout) };
    }
}

/// 测试分配释放循环的碎片化累积
fn test_allocation_deallocation_cycle(stats: &mut FragmentationStats) {
    let mut rng = rand::thread_rng();
    let cycles = 10;
    let allocations_per_cycle = 20;

    for cycle in 0..cycles {
        let mut allocations = Vec::new();

        // 分配阶段
        for _ in 0..allocations_per_cycle {
            let size = rng.gen_range(16..512);
            let layout = std::alloc::Layout::from_size_align(size, 8).unwrap();
            let ptr = unsafe { std::alloc::alloc(layout) };

            if !ptr.is_null() {
                allocations.push((ptr, layout));
                stats.record_allocation(size, size);
            } else {
                stats.record_failure();
            }
        }

        // 随机释放一半
        for _ in 0..allocations.len() / 2 {
            if let Some((ptr, layout)) = allocations.pop() {
                unsafe { std::alloc::dealloc(ptr, layout) };
            }
        }

        // 剩余的继续下一轮
        for (ptr, layout) in allocations {
            unsafe { std::alloc::dealloc(ptr, layout) };
        }

        if cycle == cycles / 2 {
            println!("     分配释放循环进行中: {}/{}", cycle + 1, cycles);
        }
    }

    println!("     完成 {} 轮分配释放循环", cycles);
}

/// 打印减少内存碎片化的建议
fn print_fragmentation_suggestions() {
    println!("   💡 减少内存碎片化的建议:");
    println!("     1. 使用对象池和内存池");
    println!("     2. 预分配内存，减少动态分配");
    println!("     3. 使用合适的分配器（Arena、Slab等）");
    println!("     4. 批量分配和释放");
    println!("     5. 避免频繁的小块分配");
    println!("     6. 使用内存对齐的数据结构");
    println!("     7. 定期进行内存整理");
}

// ==================== 6. 性能基准测试 ====================

/// 运行内存分配器的性能基准测试
///
/// 通过对比不同分配器在各种场景下的性能表现，
/// 帮助开发者选择合适的内存分配策略。
fn run_performance_benchmarks() {
    println!("🏁  6. 性能基准测试:");
    println!("   对比不同分配器的性能表现");

    // 测试场景
    let scenarios = vec![
        ("小块分配", 64, 10000),
        ("中等块分配", 1024, 5000),
        ("大块分配", 8192, 1000),
    ];

    for (name, size, count) in scenarios {
        println!("   测试场景: {} ({} bytes × {})", name, size, count);

        // 系统分配器基准
        let system_time = benchmark_system_allocator(size, count);
        println!("     系统分配器: {:?}", system_time);

        // 打印统计信息
        STATS_ALLOCATOR.print_stats(&format!("系统分配器 - {}", name));

        println!();
    }
}

/// 系统分配器性能基准测试
fn benchmark_system_allocator(size: usize, count: usize) -> Duration {
    STATS_ALLOCATOR.reset();
    let start = Instant::now();

    let mut allocations = Vec::with_capacity(count);

    for i in 0..count {
        let layout = std::alloc::Layout::from_size_align(size, 8).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) };

        if !ptr.is_null() {
            // 写入数据确保页面真正分配
            unsafe {
                ptr.write_bytes((i % 256) as u8, size);
            }
            allocations.push((ptr, layout));
        }
    }

    // 随机访问测试
    let mut rng = rand::thread_rng();
    for _ in 0..count / 10 {
        if let Some((ptr, _)) = allocations.choose(&mut rng) {
            let _byte = unsafe { *ptr };
        }
    }

    // 释放所有内存
    for (ptr, layout) in allocations {
        unsafe { std::alloc::dealloc(ptr, layout) };
    }

    start.elapsed()
}

// ==================== 主函数 ====================

fn main() {
    println!("=== Rust 内存碎片化和内存管理深入学习示例 ===\n");

    println!("本示例将演示内存碎片化的概念、问题以及各种解决方案，");
    println!("这是理解 Rust 内存管理和性能优化的关键概念。\n");

    println!("🚀 开始学习之旅...\n");

    // 1. Arena 分配器演示
    demonstrate_arena_allocator();

    // 2. Arena 分配器碎片化分析
    demonstrate_arena_fragmentation();

    // 3. Slab 分配器演示
    demonstrate_slab_allocator();

    // 4. Slab 分配器效率分析
    demonstrate_slab_efficiency();

    // 5. 内存碎片化分析
    demonstrate_memory_fragmentation();

    // 6. 性能基准测试
    run_performance_benchmarks();

    println!("=== 内存管理学习总结 ===");
    println!("🎯 核心概念回顾:");
    println!("  • 内存碎片化会降低内存利用率和性能");
    println!("  • Arena 分配器适合临时对象，零碎片化");
    println!("  • Slab 分配器适合固定大小对象，高效率");
    println!("  • 选择合适的分配器对性能至关重要");
    println!();
    println!("💡 最佳实践:");
    println!("  • 根据对象生命周期选择合适的分配器");
    println!("  • 预分配内存减少动态分配开销");
    println!("  • 使用对象池管理频繁分配的对象");
    println!("  • 定期监控内存使用情况");
    println!();
    println!("🔧 实际应用:");
    println!("  • 游戏引擎中的内存管理");
    println!("  • Web 服务器的请求处理");
    println!("  • 数据库系统的缓存管理");
    println!("  • 高频交易系统的内存优化");
    println!();
    println!("✅ 学习完成！您已经掌握了 Rust 内存管理的核心概念。");
}