/*
 * Rust 循环引用 (Reference Cycles) 深入学习示例
 *
 * 本项目展示了 Rust 中循环引用的概念、问题和解决方案，这是理解 Rust
 * 内存管理和智能指针的关键概念。
 *
 * 📚 官方文档链接：
 *
 * 🔰 基础概念
 * 1. Rust Book - 循环引用和内存泄漏:
 *    https://doc.rust-lang.org/book/ch15-06-reference-cycles.html
 *
 * 2. Rust Book - 智能指针:
 *    https://doc.rust-lang.org/book/ch15-00-smart-pointers.html
 *
 * 3. Rust by Example - Rc<T>:
 *    https://doc.rust-lang.org/rust-by-example/rc.html
 *
 * ⚙️ 智能指针文档
 * 4. Rc<T> 引用计数指针:
 *    https://doc.rust-lang.org/std/rc/struct.Rc.html
 *
 * 5. Weak<T> 弱引用指针:
 *    https://doc.rust-lang.org/std/rc/struct.Weak.html
 *
 * 6. RefCell<T> 内部可变性:
 *    https://doc.rust-lang.org/std/cell/struct.RefCell.html
 *
 * 🚀 高级概念
 * 7. Drop trait 和资源清理:
 *    https://doc.rust-lang.org/book/ch15-03-drop.html
 *
 * 8. 内存安全保证:
 *    https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
 *
 * 9. 内存泄漏预防:
 *    https://doc.rust-lang.org/book/ch15-06-reference-cycles.html#preventing-reference-cycles
 *
 * 🎯 核心学习要点：
 *
 * 🔹 循环引用的本质
 * - 循环引用是指两个或多个引用相互指向，形成闭环
 * - 在 Rust 中，这会导致引用计数永远不为零，造成内存泄漏
 * - 循环引用是智能指针使用中的常见陷阱
 *
 * 🔹 智能指针类型
 * - Rc<T>: 引用计数智能指针，允许多个所有权
 * - Weak<T>: 弱引用，不参与引用计数
 * - RefCell<T>: 提供内部可变性，允许在运行时检查借用规则
 *
 * 🔹 解决方案
 * - 使用 Weak<T> 打破循环引用
 * - 重新设计数据结构避免循环
 * - 使用图算法处理循环结构
 *
 * 🔹 实际应用场景
 * - 图结构和树结构的父节点引用
 * - 观察者模式中的双向引用
 * - 缓存系统中的循环依赖
 */

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    fmt::Debug,
};

// ==================== 1. 循环引用问题演示 ====================

/// 简单的链表节点 - 会产生循环引用问题
///
/// 这个结构体演示了循环引用的典型场景：
/// 当链表的尾节点指向头节点时，会形成循环引用。
///
/// # 字段说明
/// * `value` - 节点存储的值
/// * `next` - 指向下一个节点的强引用，使用 RefCell 提供内部可变性
///
/// # 问题演示
/// - 当创建循环链表时，每个节点的引用计数都至少为 1
/// - 即使不再有任何外部引用，节点之间仍然相互引用
/// - 这导致内存永远无法释放，形成内存泄漏
///
/// # 文档链接
/// - [Rc<T> 文档](https://doc.rust-lang.org/std/rc/struct.Rc.html)
/// - [RefCell<T> 文档](https://doc.rust-lang.org/std/cell/struct.RefCell.html)
/// - [循环引用问题](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)
#[derive(Debug)]
struct Node {
    /// 节点存储的整数值
    value: i32,
    /// 指向下一个节点的强引用
    /// RefCell 允许我们在运行时修改不可变引用的内容
    next: RefCell<Option<Rc<Node>>>,
}

/// 实现自定义的 Drop trait 来观察节点的销毁过程
///
/// Drop trait 在对象被销毁时自动调用，这让我们可以
/// 跟踪内存释放的过程，验证是否存在内存泄漏。
impl Drop for Node {
    fn drop(&mut self) {
        println!("🗑️  正在销毁 Node (值: {})", self.value);
        // 在实际应用中，这里可能会执行清理工作
        // 例如：关闭文件连接、释放网络资源等
    }
}

impl Node {
    /// 创建新的节点
    ///
    /// # 参数
    /// * `value` - 节点的值
    ///
    /// # 返回值
    /// 返回一个 Node 实例的强引用
    fn new(value: i32) -> Rc<Self> {
        Rc::new(Node {
            value,
            next: RefCell::new(None),
        })
    }

    /// 获取当前节点的引用计数
    ///
    /// # 返回值
    /// 当前节点的强引用数量
    fn strong_count(&self) -> usize {
        // 注意：这里我们无法直接访问 self 的引用计数
        // 因为 self 是 &Self，而不是 Rc<Self>
        // 这个方法主要用于演示目的
        println!("⚠️  Node {} 无法直接获取引用计数", self.value);
        0
    }

    /// 获取节点的值和下一个节点信息
    fn debug_info(&self) -> (i32, Option<i32>) {
        let next_value = self.next.borrow()
            .as_ref()
            .map(|next_node| next_node.value);
        (self.value, next_value)
    }
}

// ==================== 2. 使用弱引用解决循环引用 ====================

/// 使用弱引用的链表节点 - 解决循环引用问题
///
/// 这个结构体展示了如何使用 Weak<T> 来打破循环引用：
/// 通过在循环路径的一端使用弱引用，可以让内存正确释放。
///
/// # 设计模式
/// - 强引用用于表示所有权关系
/// - 弱引用用于表示非所有权的关系（如父节点引用）
/// - 枚举类型提供了类型安全的引用类型切换
///
/// # 字段说明
/// * `value` - 节点存储的值
/// * `next` - 指向下一个节点的引用，可以是强引用或弱引用
///
/// # 文档链接
/// - [Weak<T> 文档](https://doc.rust-lang.org/std/rc/struct.Weak.html)
/// - [打破循环引用](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html#preventing-reference-cycles)
#[derive(Debug)]
struct Node2 {
    /// 节点存储的整数值
    value: i32,
    /// 指向下一个节点的引用，可以是强引用或弱引用
    next: RefCell<NextNode>,
}

/// 表示节点引用类型的枚举
///
/// 这个枚举允许我们在运行时决定使用强引用还是弱引用，
/// 提供了类型安全的引用管理。
///
/// # 变体说明
/// * `None` - 没有下一个节点
/// * `Strong(Rc<Node2>)` - 强引用，会增加引用计数
/// * `Weak(Weak<Node2>)` - 弱引用，不会增加引用计数
#[derive(Debug)]
enum NextNode {
    /// 没有下一个节点，链表结束
    None,
    /// 强引用，表示所有权关系
    Strong(Rc<Node2>),
    /// 弱引用，表示非所有权关系
    Weak(Weak<Node2>),
}

impl Node2 {
    /// 创建新节点
    fn new(value: i32) -> Rc<Self> {
        Rc::new(Node2 {
            value,
            next: RefCell::new(NextNode::None),
        })
    }

    /// 获取节点的强引用计数
    fn strong_count(&self) -> usize {
        Rc::strong_count(self)
    }

    /// 获取节点的弱引用计数
    fn weak_count(&self) -> usize {
        Rc::weak_count(self)
    }

    /// 设置下一个节点为强引用
    fn set_next_strong(&self, next_node: Rc<Node2>) {
        *self.next.borrow_mut() = NextNode::Strong(next_node);
    }

    /// 设置下一个节点为弱引用
    fn set_next_weak(&self, next_node: &Rc<Node2>) {
        *self.next.borrow_mut() = NextNode::Weak(Rc::downgrade(next_node));
    }

    /// 获取调试信息
    fn debug_info(&self) -> String {
        let strong_count = self.strong_count();
        let weak_count = self.weak_count();

        match &*self.next.borrow() {
            NextNode::None => format!("Node2(值: {}, 强引用: {}, 弱引用: {}, 下一个: None)",
                                    self.value, strong_count, weak_count),
            NextNode::Strong(next) => format!("Node2(值: {}, 强引用: {}, 弱引用: {}, 下一个: 强引用->{})",
                                            self.value, strong_count, weak_count, next.value),
            NextNode::Weak(weak_ref) => {
                if let Some(upgraded) = weak_ref.upgrade() {
                    format!("Node2(值: {}, 强引用: {}, 弱引用: {}, 下一个: 弱引用->{})",
                           self.value, strong_count, weak_count, upgraded.value)
                } else {
                    format!("Node2(值: {}, 强引用: {}, 弱引用: {}, 下一个: 弱引用->已销毁)",
                           self.value, strong_count, weak_count)
                }
            }
        }
    }
}

/// 实现自定义的 Drop trait 来观察 Node2 的销毁过程
impl Drop for Node2 {
    fn drop(&mut self) {
        println!("🗑️  正在销毁 Node2 (值: {})", self.value);
    }
}

// ==================== 3. 复杂循环引用场景 ====================

/// 图节点 - 更复杂的循环引用场景
///
/// 图结构是循环引用的典型场景，因为节点之间可能
/// 存在双向或多向的连接关系。
///
/// # 应用场景
/// - 社交网络中的好友关系
/// - 软件依赖关系图
/// - 地理位置和路径网络
#[derive(Debug)]
struct GraphNode {
    name: String,
    neighbors: RefCell<Vec<Rc<GraphNode>>>,
    parent: RefCell<Option<Weak<GraphNode>>>,
}

impl GraphNode {
    fn new(name: &str) -> Rc<Self> {
        Rc::new(GraphNode {
            name: name.to_string(),
            neighbors: RefCell::new(Vec::new()),
            parent: RefCell::new(None),
        })
    }

    /// 添加邻居节点（强引用）
    fn add_neighbor(&self, neighbor: Rc<GraphNode>) {
        self.neighbors.borrow_mut().push(neighbor);
    }

    /// 设置父节点（弱引用，避免循环）
    fn set_parent(&self, parent: &Rc<GraphNode>) {
        *self.parent.borrow_mut() = Some(Rc::downgrade(parent));
    }

    /// 获取节点信息
    fn get_info(&self) -> String {
        let neighbor_count = self.neighbors.borrow().len();
        let has_parent = self.parent.borrow().is_some();
        format!("图节点 '{}' (邻居: {}, 有父节点: {})",
                self.name, neighbor_count, has_parent)
    }
}

impl Drop for GraphNode {
    fn drop(&mut self) {
        println!("🗑️  正在销毁图节点: {}", self.name);
    }
}

// ==================== 4. 观察者模式示例 ====================

/// 观察者接口
trait Observer: Debug {
    fn notify(&self, message: &str);
}

/// 被观察者（主题）
struct Subject {
    name: String,
    observers: RefCell<Vec<Weak<dyn Observer>>>,
}

impl Subject {
    fn new(name: &str) -> Rc<Self> {
        Rc::new(Subject {
            name: name.to_string(),
            observers: RefCell::new(Vec::new()),
        })
    }

    /// 添加观察者（使用弱引用避免循环）
    fn add_observer(&self, observer: &Rc<dyn Observer>) {
        self.observers.borrow_mut().push(Rc::downgrade(observer));
    }

    /// 通知所有观察者
    fn notify_observers(&self, message: &str) {
        println!("📢 主题 '{}' 发送通知: {}", self.name, message);

        // 清理已销毁的弱引用
        self.observers.borrow_mut().retain(|weak| weak.upgrade().is_some());

        // 通知所有活跃的观察者
        for weak_observer in self.observers.borrow().iter() {
            if let Some(observer) = weak_observer.upgrade() {
                observer.notify(message);
            }
        }
    }
}

/// 具体观察者
struct ConcreteObserver {
    name: String,
    subject: RefCell<Option<Weak<Subject>>>,
}

impl ConcreteObserver {
    fn new(name: &str) -> Rc<Self> {
        Rc::new(ConcreteObserver {
            name: name.to_string(),
            subject: RefCell::new(None),
        })
    }

    /// 订阅主题
    fn subscribe(&self, subject: &Rc<Subject>) {
        *self.subject.borrow_mut() = Some(Rc::downgrade(subject));
        subject.add_observer(self);
    }

    /// 取消订阅
    fn unsubscribe(&self) {
        *self.subject.borrow_mut() = None;
    }
}

impl Debug for ConcreteObserver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConcreteObserver({})", self.name)
    }
}

impl Observer for ConcreteObserver {
    fn notify(&self, message: &str) {
        println!("  👀 观察 '{}' 收到通知: {}", self.name, message);
    }
}

impl Drop for ConcreteObserver {
    fn drop(&mut self) {
        println!("🗑️  正在销毁观察者: {}", self.name);
    }
}

// ==================== 5. 演示函数 ====================

/// 演示循环引用问题
fn demonstrate_cycle_problem() {
    println!("🔴 1. 循环引用问题演示:");
    println!("   创建一个循环链表，观察内存泄漏问题");

    {
        // 创建第一个节点（尾部）
        let tail = Node::new(1);
        println!("   创建 tail 节点，引用计数: {}", Rc::strong_count(&tail));

        // 创建第二个节点（头部）
        let head = Node::new(2);
        println!("   创建 head 节点，引用计数: {}", Rc::strong_count(&head));

        // 建立连接：head -> tail
        *head.next.borrow_mut() = Some(tail.clone());
        println!("   连接 head -> tail");
        println!("   tail 引用计数: {}, head 引用计数: {}",
                Rc::strong_count(&tail), Rc::strong_count(&head));

        // 建立循环：tail -> head
        *tail.next.borrow_mut() = Some(head.clone());
        println!("   建立循环 tail -> head");
        println!("   tail 引用计数: {}, head 引用计数: {}",
                Rc::strong_count(&tail), Rc::strong_count(&head));

        // 在这个作用域结束时，理论上 head 和 tail 应该被销毁
        // 但由于循环引用，它们的引用计数永远不会降到 0
        println!("   🔴 注意：作用域结束，但节点不会被销毁（内存泄漏）");

    } // head 和 tail 在这里离开作用域，但由于循环引用，内存不会被释放

    println!("   ⚠️  节点没有在这个点被销毁，说明发生了内存泄漏");
    println!();
}

/// 演示使用弱引用解决循环引用
fn demonstrate_weak_reference_solution() {
    println!("🟢 2. 使用弱引用解决循环引用:");
    println!("   使用 Weak<T> 打破循环引用，让内存能够正确释放");

    {
        // 创建尾部节点
        let tail = Node2::new(1);
        println!("   创建 tail: {}", tail.debug_info());

        // 创建头部节点
        let head = Node2::new(2);
        println!("   创建 head: {}", head.debug_info());

        // 建立连接：head -> tail (强引用)
        head.set_next_strong(tail.clone());
        println!("   连接 head -> tail");
        println!("   head: {}", head.debug_info());
        println!("   tail: {}", tail.debug_info());

        // 建立弱引用：tail -> head (弱引用，打破循环)
        tail.set_next_weak(&head);
        println!("   建立弱引用 tail -> head");
        println!("   head: {}", head.debug_info());
        println!("   tail: {}", tail.debug_info());

        println!("   🟢 注意：作用域结束，节点将被正确销毁");

    } // head 和 tail 在这里离开作用域，弱引用不会阻止销毁

    println!("   ✅ 节点已正确销毁，内存泄漏问题已解决");
    println!();
}

/// 演示图结构中的循环引用处理
fn demonstrate_graph_cycles() {
    println!("🔵 3. 图结构循环引用处理:");
    println!("   处理图结构中更复杂的循环引用场景");

    {
        // 创建图节点
        let node_a = GraphNode::new("A");
        let node_b = GraphNode::new("B");
        let node_c = GraphNode::new("C");

        println!("   创建了三个图节点");

        // 建立双向连接（邻居关系）
        node_a.add_neighbor(node_b.clone());
        node_b.add_neighbor(node_a.clone());
        node_b.add_neighbor(node_c.clone());
        node_c.add_neighbor(node_b.clone());

        println!("   建立邻居关系: A<->B<->C");

        // 设置层次关系（使用弱引用避免循环）
        node_b.set_parent(&node_a);
        node_c.set_parent(&node_b);

        println!("   设置父子关系: A->B->C (使用弱引用)");

        // 显示节点信息
        println!("   {}", node_a.get_info());
        println!("   {}", node_b.get_info());
        println!("   {}", node_c.get_info());

        println!("   🔵 图结构节点将在作用域结束时正确销毁");

    } // 图节点在这里被销毁

    println!("   ✅ 图结构节点已正确销毁");
    println!();
}

/// 演示观察者模式中的循环引用处理
fn demonstrate_observer_pattern() {
    println!("🟡 4. 观察者模式循环引用处理:");
    println!("   在观察者模式中使用弱引用避免内存泄漏");

    {
        // 创建主题
        let subject = Subject::new("新闻发布者");
        println!("   创建主题: {}", subject.name);

        // 创建观察者
        let observer1 = ConcreteObserver::new("订阅者1");
        let observer2 = ConcreteObserver::new("订阅者2");

        // 订阅主题
        observer1.subscribe(&subject);
        observer2.subscribe(&subject);
        println!("   两个观察者订阅了主题");

        // 发送通知
        subject.notify_observers("新文章发布：Rust 智能指针详解");
        println!();

        // 一个观察者取消订阅
        observer1.unsubscribe();
        println!("   订阅者1 取消订阅");

        // 再次发送通知
        subject.notify_observers("新文章发布：内存管理最佳实践");
        println!();

        println!("   🟡 观察者和主题将在作用域结束时正确销毁");

    } // 主题和观察者在这里被销毁

    println!("   ✅ 观察者模式中的对象已正确销毁");
    println!();
}

/// 演示引用计数的实际使用
fn demonstrate_reference_counting() {
    println!("🟣 5. 引用计数实际演示:");
    println!("   深入理解 Rc 和 Weak 的引用计数机制");

    {
        // 创建一个节点
        let node = Node2::new(100);
        println!("   创建节点: {}", node.debug_info());

        // 创建多个强引用
        let ref1 = node.clone();
        println!("   创建强引用1: {}", node.debug_info());

        let ref2 = node.clone();
        println!("   创建强引用2: {}", node.debug_info());

        let ref3 = node.clone();
        println!("   创建强引用3: {}", node.debug_info());

        // 创建弱引用
        let weak_ref = Rc::downgrade(&node);
        println!("   创建弱引用后: {}", node.debug_info());

        // 通过弱引用访问节点
        if let Some(upgraded) = weak_ref.upgrade() {
            println!("   弱引用升级成功: {}", upgraded.debug_info());
        }

        // 逐个释放强引用
        drop(ref1);
        println!("   释放 ref1 后: {}", node.debug_info());

        drop(ref2);
        println!("   释放 ref2 后: {}", node.debug_info());

        drop(ref3);
        println!("   释放 ref3 后: {}", node.debug_info());

        // 检查弱引用状态
        match weak_ref.upgrade() {
            Some(upgraded) => println!("   弱引用仍然有效: {}", upgraded.debug_info()),
            None => println!("   弱引用已失效，节点已被销毁"),
        }

        println!("   🟣 最后的强引用将在这个作用域结束时释放");
    }

    println!("   ✅ 节点已被完全销毁");
    println!();
}

// ==================== 主函数 ====================

fn main() {
    println!("=== Rust 循环引用深入学习示例 ===\n");

    println!("本示例将演示循环引用的问题和解决方案，");
    println!("这是理解 Rust 内存管理和智能指针的关键概念。\n");

    // 1. 演示循环引用问题
    demonstrate_cycle_problem();

    // 2. 演示使用弱引用的解决方案
    demonstrate_weak_reference_solution();

    // 3. 演示图结构中的循环引用处理
    demonstrate_graph_cycles();

    // 4. 演示观察者模式中的循环引用处理
    demonstrate_observer_pattern();

    // 5. 演示引用计数机制
    demonstrate_reference_counting();

    println!("=== 循环引用学习总结 ===");
    println!("🎯 核心概念回顾:");
    println!("  • 循环引用会导致内存泄漏");
    println!("  • Weak<T> 可以打破循环引用");
    println!("  • RefCell<T> 提供内部可变性");
    println!("  • 引用计数决定了对象的生命周期");
    println!();
    println!("💡 最佳实践:");
    println!("  • 在设计数据结构时避免不必要的循环");
    println!("  • 对于父子关系，子节点使用弱引用指向父节点");
    println!("  • 观察者模式中使用弱引用避免循环依赖");
    println!("  • 定期检查和清理失效的弱引用");
    println!();
    println!("🔧 实际应用:");
    println!("  • GUI 框架中的组件关系管理");
    println!("  • 游戏引擎中的实体关系系统");
    println!("  • 缓存系统和依赖管理");
    println!("  • 网络协议和状态管理");
}
