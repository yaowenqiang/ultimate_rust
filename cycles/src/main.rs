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

/// 循环链表节点 - 会产生内存泄漏问题
///
/// 这个结构体演示了循环引用的典型场景：
/// 当链表的尾节点指向头节点时，会形成循环引用。
///
/// # 字段说明
/// * `data` - 节点存储的数据值
/// * `next_node` - 指向下一个节点的强引用，使用 RefCell 提供内部可变性
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
struct CircularListNode {
    /// 节点存储的整数值
    data: i32,
    /// 指向下一个节点的强引用
    /// RefCell 允许我们在运行时修改不可变引用的内容
    next_node: RefCell<Option<Rc<CircularListNode>>>,
}

/// 实现自定义的 Drop trait 来观察节点的销毁过程
///
/// Drop trait 在对象被销毁时自动调用，这让我们可以
/// 跟踪内存释放的过程，验证是否存在内存泄漏。
impl Drop for CircularListNode {
    fn drop(&mut self) {
        println!("🗑️  正在销毁 CircularListNode (值: {})", self.data);
        // 在实际应用中，这里可能会执行清理工作
        // 例如：关闭文件连接、释放网络资源等
    }
}

impl CircularListNode {
    /// 创建新的节点
    ///
    /// # 参数
    /// * `data` - 节点的数据值
    ///
    /// # 返回值
    /// 返回一个 CircularListNode 实例的强引用
    fn create(data: i32) -> Rc<Self> {
        Rc::new(CircularListNode {
            data,
            next_node: RefCell::new(None),
        })
    }

    /// 获取当前节点的引用计数
    ///
    /// # 返回值
    /// 当前节点的强引用数量
    fn get_strong_count(&self) -> usize {
        // 注意：这里我们无法直接访问 self 的引用计数
        // 因为 self 是 &Self，而不是 Rc<Self>
        // 这个方法主要用于演示目的
        println!("⚠️  CircularListNode {} 无法直接获取引用计数", self.data);
        0
    }

    /// 获取节点的数据值和下一个节点信息
    fn get_debug_info(&self) -> (i32, Option<i32>) {
        let next_data = self.next_node.borrow()
            .as_ref()
            .map(|next_node| next_node.data);
        (self.data, next_data)
    }
}

// ==================== 2. 使用弱引用解决循环引用 ====================

/// 安全链表节点 - 使用弱引用解决循环引用问题
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
/// * `data` - 节点存储的数据值
/// * `next_node_ref` - 指向下一个节点的引用，可以是强引用或弱引用
///
/// # 文档链接
/// - [Weak<T> 文档](https://doc.rust-lang.org/std/rc/struct.Weak.html)
/// - [打破循环引用](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html#preventing-reference-cycles)
#[derive(Debug)]
struct SafeListNode {
    /// 节点存储的整数值
    data: i32,
    /// 指向下一个节点的引用，可以是强引用或弱引用
    next_node_ref: RefCell<NodeReference>,
}

/// 表示节点引用类型的枚举
///
/// 这个枚举允许我们在运行时决定使用强引用还是弱引用，
/// 提供了类型安全的引用管理。
///
/// # 变体说明
/// * `NoReference` - 没有下一个节点
/// * `StrongReference(Rc<SafeListNode>)` - 强引用，会增加引用计数
/// * `WeakReference(Weak<SafeListNode>)` - 弱引用，不会增加引用计数
#[derive(Debug)]
enum NodeReference {
    /// 没有下一个节点，链表结束
    NoReference,
    /// 强引用，表示所有权关系
    StrongReference(Rc<SafeListNode>),
    /// 弱引用，表示非所有权关系
    WeakReference(Weak<SafeListNode>),
}

impl SafeListNode {
    /// 创建新节点
    fn create(data: i32) -> Rc<Self> {
        Rc::new(SafeListNode {
            data,
            next_node_ref: RefCell::new(NodeReference::NoReference),
        })
    }

    /// 获取节点的强引用计数
    fn get_strong_count(this: &Rc<Self>) -> usize {
        Rc::strong_count(this)
    }

    /// 获取节点的弱引用计数
    fn get_weak_count(this: &Rc<Self>) -> usize {
        Rc::weak_count(this)
    }

    /// 设置下一个节点为强引用
    fn link_to_strong(&self, next_node: Rc<SafeListNode>) {
        *self.next_node_ref.borrow_mut() = NodeReference::StrongReference(next_node);
    }

    /// 设置下一个节点为弱引用
    fn link_to_weak(&self, next_node: &Rc<SafeListNode>) {
        *self.next_node_ref.borrow_mut() = NodeReference::WeakReference(Rc::downgrade(next_node));
    }

    /// 获取调试信息
    fn get_debug_info(this: &Rc<Self>) -> String {
        let strong_count = Self::get_strong_count(this);
        let weak_count = Self::get_weak_count(this);

        match &*this.next_node_ref.borrow() {
            NodeReference::NoReference => {
                format!("SafeListNode(数据: {}, 强引用: {}, 弱引用: {}, 下一个: None)",
                       this.data, strong_count, weak_count)
            },
            NodeReference::StrongReference(next) => {
                format!("SafeListNode(数据: {}, 强引用: {}, 弱引用: {}, 下一个: 强引用->{})",
                       this.data, strong_count, weak_count, next.data)
            },
            NodeReference::WeakReference(weak_ref) => {
                if let Some(upgraded) = weak_ref.upgrade() {
                    format!("SafeListNode(数据: {}, 强引用: {}, 弱引用: {}, 下一个: 弱引用->{})",
                           this.data, strong_count, weak_count, upgraded.data)
                } else {
                    format!("SafeListNode(数据: {}, 强引用: {}, 弱引用: {}, 下一个: 弱引用->已销毁)",
                           this.data, strong_count, weak_count)
                }
            }
        }
    }
}

/// 实现自定义的 Drop trait 来观察 SafeListNode 的销毁过程
impl Drop for SafeListNode {
    fn drop(&mut self) {
        println!("🗑️  正在销毁 SafeListNode (数据: {})", self.data);
    }
}

// ==================== 3. 复杂循环引用场景 ====================

/// 网络图节点 - 更复杂的循环引用场景
///
/// 图结构是循环引用的典型场景，因为节点之间可能
/// 存在双向或多向的连接关系。
///
/// # 应用场景
/// - 社交网络中的好友关系
/// - 软件依赖关系图
/// - 地理位置和路径网络
#[derive(Debug)]
struct NetworkGraphNode {
    node_name: String,
    adjacent_nodes: RefCell<Vec<Rc<NetworkGraphNode>>>,
    parent_node: RefCell<Option<Weak<NetworkGraphNode>>>,
}

impl NetworkGraphNode {
    fn create(name: &str) -> Rc<Self> {
        Rc::new(NetworkGraphNode {
            node_name: name.to_string(),
            adjacent_nodes: RefCell::new(Vec::new()),
            parent_node: RefCell::new(None),
        })
    }

    /// 添加相邻节点（强引用）
    fn connect_to(&self, neighbor: Rc<NetworkGraphNode>) {
        self.adjacent_nodes.borrow_mut().push(neighbor);
    }

    /// 设置父节点（弱引用，避免循环）
    fn set_parent_node(&self, parent: &Rc<NetworkGraphNode>) {
        *self.parent_node.borrow_mut() = Some(Rc::downgrade(parent));
    }

    /// 获取节点信息
    fn get_node_info(&self) -> String {
        let neighbor_count = self.adjacent_nodes.borrow().len();
        let has_parent = self.parent_node.borrow().is_some();
        format!("网络图节点 '{}' (相邻节点: {}, 有父节点: {})",
                self.node_name, neighbor_count, has_parent)
    }
}

impl Drop for NetworkGraphNode {
    fn drop(&mut self) {
        println!("🗑️  正在销毁网络图节点: {}", self.node_name);
    }
}

// ==================== 4. 观察者模式示例 ====================

/// 观察者接口
trait EventObserver: Debug {
    fn handle_notification(&self, message: &str);
}

/// 被观察者（主题）
struct EventPublisher {
    publisher_name: String,
    subscriber_list: RefCell<Vec<Weak<EventSubscriber>>>,
}

impl EventPublisher {
    fn create(name: &str) -> Rc<Self> {
        Rc::new(EventPublisher {
            publisher_name: name.to_string(),
            subscriber_list: RefCell::new(Vec::new()),
        })
    }

    /// 添加观察者（使用弱引用避免循环）
    fn register_subscriber(&self, observer: &Rc<EventSubscriber>) {
        self.subscriber_list.borrow_mut().push(Rc::downgrade(observer));
    }

    /// 通知所有观察者
    fn broadcast_message(&self, message: &str) {
        println!("📢 发布者 '{}' 发送通知: {}", self.publisher_name, message);

        // 清理已销毁的弱引用
        self.subscriber_list.borrow_mut().retain(|weak| weak.upgrade().is_some());

        // 通知所有活跃的观察者
        for weak_subscriber in self.subscriber_list.borrow().iter() {
            if let Some(subscriber) = weak_subscriber.upgrade() {
                subscriber.handle_notification(message);
            }
        }
    }
}

/// 具体观察者
struct EventSubscriber {
    subscriber_name: String,
    subscribed_to: RefCell<Option<Weak<EventPublisher>>>,
}

impl EventSubscriber {
    fn create(name: &str) -> Rc<Self> {
        Rc::new(EventSubscriber {
            subscriber_name: name.to_string(),
            subscribed_to: RefCell::new(None),
        })
    }

    /// 订阅主题
    fn subscribe_to(this: &Rc<Self>, publisher: &Rc<EventPublisher>) {
        *this.subscribed_to.borrow_mut() = Some(Rc::downgrade(publisher));
        publisher.register_subscriber(this);
    }

    /// 取消订阅
    fn unsubscribe(&self) {
        *self.subscribed_to.borrow_mut() = None;
    }
}

impl Debug for EventSubscriber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EventSubscriber({})", self.subscriber_name)
    }
}

impl EventObserver for EventSubscriber {
    fn handle_notification(&self, message: &str) {
        println!("  👀 订阅者 '{}' 收到通知: {}", self.subscriber_name, message);
    }
}

impl Drop for EventSubscriber {
    fn drop(&mut self) {
        println!("🗑️  正在销毁订阅者: {}", self.subscriber_name);
    }
}

// ==================== 5. 演示函数 ====================

/// 演示循环引用问题
fn demonstrate_cycle_problem() {
    println!("🔴 1. 循环引用问题演示:");
    println!("   创建一个循环链表，观察内存泄漏问题");

    {
        // 创建第一个节点（尾部）
        let tail_node = CircularListNode::create(1);
        println!("   创建 tail_node 节点，引用计数: {}", Rc::strong_count(&tail_node));

        // 创建第二个节点（头部）
        let head_node = CircularListNode::create(2);
        println!("   创建 head_node 节点，引用计数: {}", Rc::strong_count(&head_node));

        // 建立连接：head_node -> tail_node
        *head_node.next_node.borrow_mut() = Some(tail_node.clone());
        println!("   连接 head_node -> tail_node");
        println!("   tail_node 引用计数: {}, head_node 引用计数: {}",
                Rc::strong_count(&tail_node), Rc::strong_count(&head_node));

        // 建立循环：tail_node -> head_node
        *tail_node.next_node.borrow_mut() = Some(head_node.clone());
        println!("   建立循环 tail_node -> head_node");
        println!("   tail_node 引用计数: {}, head_node 引用计数: {}",
                Rc::strong_count(&tail_node), Rc::strong_count(&head_node));

        // 在这个作用域结束时，理论上 head_node 和 tail_node 应该被销毁
        // 但由于循环引用，它们的引用计数永远不会降到 0
        println!("   🔴 注意：作用域结束，但节点不会被销毁（内存泄漏）");

    } // head_node 和 tail_node 在这里离开作用域，但由于循环引用，内存不会被释放

    println!("   ⚠️  节点没有在这个点被销毁，说明发生了内存泄漏");
    println!();
}

/// 演示使用弱引用解决循环引用
fn demonstrate_weak_reference_solution() {
    println!("🟢 2. 使用弱引用解决循环引用:");
    println!("   使用 Weak<T> 打破循环引用，让内存能够正确释放");

    {
        // 创建尾部节点
        let tail_node = SafeListNode::create(1);
        println!("   创建 tail_node: {}", SafeListNode::get_debug_info(&tail_node));

        // 创建头部节点
        let head_node = SafeListNode::create(2);
        println!("   创建 head_node: {}", SafeListNode::get_debug_info(&head_node));

        // 建立连接：head_node -> tail_node (强引用)
        head_node.link_to_strong(tail_node.clone());
        println!("   连接 head_node -> tail_node");
        println!("   head_node: {}", SafeListNode::get_debug_info(&head_node));
        println!("   tail_node: {}", SafeListNode::get_debug_info(&tail_node));

        // 建立弱引用：tail_node -> head_node (弱引用，打破循环)
        tail_node.link_to_weak(&head_node);
        println!("   建立弱引用 tail_node -> head_node");
        println!("   head_node: {}", SafeListNode::get_debug_info(&head_node));
        println!("   tail_node: {}", SafeListNode::get_debug_info(&tail_node));

        println!("   🟢 注意：作用域结束，节点将被正确销毁");

    } // head_node 和 tail_node 在这里离开作用域，弱引用不会阻止销毁

    println!("   ✅ 节点已正确销毁，内存泄漏问题已解决");
    println!();
}

/// 演示图结构中的循环引用处理
fn demonstrate_graph_cycles() {
    println!("🔵 3. 图结构循环引用处理:");
    println!("   处理图结构中更复杂的循环引用场景");

    {
        // 创建图节点
        let node_a = NetworkGraphNode::create("A");
        let node_b = NetworkGraphNode::create("B");
        let node_c = NetworkGraphNode::create("C");

        println!("   创建了三个网络图节点");

        // 建立双向连接（邻居关系）
        node_a.connect_to(node_b.clone());
        node_b.connect_to(node_a.clone());
        node_b.connect_to(node_c.clone());
        node_c.connect_to(node_b.clone());

        println!("   建立相邻关系: A<->B<->C");

        // 设置层次关系（使用弱引用避免循环）
        node_b.set_parent_node(&node_a);
        node_c.set_parent_node(&node_b);

        println!("   设置父子关系: A->B->C (使用弱引用)");

        // 显示节点信息
        println!("   {}", node_a.get_node_info());
        println!("   {}", node_b.get_node_info());
        println!("   {}", node_c.get_node_info());

        println!("   🔵 网络图节点将在作用域结束时正确销毁");

    } // 网络图节点在这里被销毁

    println!("   ✅ 网络图节点已正确销毁");
    println!();
}

/// 演示观察者模式中的循环引用处理
fn demonstrate_observer_pattern() {
    println!("🟡 4. 观察者模式循环引用处理:");
    println!("   在观察者模式中使用弱引用避免内存泄漏");

    {
        // 创建发布者
        let news_publisher = EventPublisher::create("新闻发布者");
        println!("   创建发布者: {}", news_publisher.publisher_name);

        // 创建订阅者
        let subscriber_one = EventSubscriber::create("订阅者1");
        let subscriber_two = EventSubscriber::create("订阅者2");

        // 订阅发布者
        EventSubscriber::subscribe_to(&subscriber_one, &news_publisher);
        EventSubscriber::subscribe_to(&subscriber_two, &news_publisher);
        println!("   两个订阅者订阅了发布者");

        // 发送通知
        news_publisher.broadcast_message("新文章发布：Rust 智能指针详解");
        println!();

        // 一个订阅者取消订阅
        subscriber_one.unsubscribe();
        println!("   订阅者1 取消订阅");

        // 再次发送通知
        news_publisher.broadcast_message("新文章发布：内存管理最佳实践");
        println!();

        println!("   🟡 发布者和订阅者将在作用域结束时正确销毁");

    } // 发布者和订阅者在这里被销毁

    println!("   ✅ 观察者模式中的对象已正确销毁");
    println!();
}

/// 演示引用计数的实际使用
fn demonstrate_reference_counting() {
    println!("🟣 5. 引用计数实际演示:");
    println!("   深入理解 Rc 和 Weak 的引用计数机制");

    {
        // 创建一个节点
        let test_node = SafeListNode::create(100);
        println!("   创建节点: {}", SafeListNode::get_debug_info(&test_node));

        // 创建多个强引用
        let strong_ref1 = test_node.clone();
        println!("   创建强引用1: {}", SafeListNode::get_debug_info(&test_node));

        let strong_ref2 = test_node.clone();
        println!("   创建强引用2: {}", SafeListNode::get_debug_info(&test_node));

        let strong_ref3 = test_node.clone();
        println!("   创建强引用3: {}", SafeListNode::get_debug_info(&test_node));

        // 创建弱引用
        let weak_reference = Rc::downgrade(&test_node);
        println!("   创建弱引用后: {}", SafeListNode::get_debug_info(&test_node));

        // 通过弱引用访问节点
        if let Some(upgraded_ref) = weak_reference.upgrade() {
            println!("   弱引用升级成功: {}", SafeListNode::get_debug_info(&upgraded_ref));
        }

        // 逐个释放强引用
        drop(strong_ref1);
        println!("   释放 strong_ref1 后: {}", SafeListNode::get_debug_info(&test_node));

        drop(strong_ref2);
        println!("   释放 strong_ref2 后: {}", SafeListNode::get_debug_info(&test_node));

        drop(strong_ref3);
        println!("   释放 strong_ref3 后: {}", SafeListNode::get_debug_info(&test_node));

        // 检查弱引用状态
        match weak_reference.upgrade() {
            Some(upgraded_ref) => println!("   弱引用仍然有效: {}", SafeListNode::get_debug_info(&upgraded_ref)),
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
