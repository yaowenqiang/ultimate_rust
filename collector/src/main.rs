// 数据收集器 (Collector) - 分布式系统监控组件
//
// 这个程序是一个系统性能数据收集器，用于：
// 1. 收集本地系统的 CPU 和内存使用情况
// 2. 通过网络将数据发送到中央服务器
// 3. 使用持久化 UUID 来标识收集器实例
//
// 相关文档：
// - sysinfo 库：https://docs.rs/sysinfo/latest/sysinfo/
// - Rust 线程：https://doc.rust-lang.org/std/thread/
// - Rust 网络编程：https://doc.rust-lang.org/std/net/
// - UUID 生成：https://docs.rs/uuid/latest/uuid/

use shared_data::{
    CollectorCommandV1, CollectorResponseV1, DATA_COLLECTOR_ADDRESS, decode_response_v1,
};
use std::collections::VecDeque;
use std::io::{Read, Write};
use std::sync::mpsc::Sender; // 多生产者单消费者通道的发送端
use std::time::Instant;
use thiserror::Error;
use uuid::Uuid;

/// 获取或生成收集器的唯一标识符
///
/// 这个函数实现了一个持久化的 UUID 机制：
/// 1. 首先检查本地是否已存在 "uuid" 文件
/// 2. 如果存在，读取其中的 UUID 值
/// 3. 如果不存在，生成新的 UUID v4 并保存到文件
///
/// 这样确保了收集器重启后仍能保持相同的标识符
///
/// # Returns
/// u128 - UUID 的 128 位数值表示
///
/// # 相关概念
/// - UUID v4: 随机生成的通用唯一标识符
/// - 文件持久化: 将数据保存到文件以便重启后恢复
///
/// 文档参考：
/// - UUID 规范：https://datatracker.ietf.org/doc/html/rfc4122
/// - Rust 文件操作：https://doc.rust-lang.org/std/fs/
fn get_uuid() -> u128 {
    let path = std::path::Path::new("uuid");
    if path.exists() {
        // 读取已存在的 UUID 文件
        let contents = std::fs::read_to_string(path).unwrap();
        contents.parse::<u128>().unwrap()
    } else {
        // 生成新的 UUID v4 并保存
        let uuid = Uuid::new_v4().as_u128();
        std::fs::write(path, uuid.to_string()).unwrap();
        uuid
    }
}

#[derive(Debug, Error)]
pub enum CollectorError {
    #[error("Unable to connect to the server")]
    UnableToConnect,
    #[error("Sending the data failed")]
    UnableToSend,
    #[error("Receive data failed")]
    UnableToReceive,
}

/// 系统性能数据收集函数
///
/// 这个函数在一个无限循环中持续收集系统性能数据：
/// 1. 初始化系统信息监控器
/// 2. 定期刷新并收集 CPU 和内存使用率
/// 3. 将数据通过通道发送给主线程
/// 4. 保持大约 1 秒的采集间隔
///
/// # 参数
/// * `tx` - 通道发送端，用于将收集的数据发送给主线程
/// * `collector_id` - 收集器的唯一标识符
///
/// # 收集的数据指标
/// - 总内存量 (total_memory)
/// - 已使用内存量 (used_memory)
/// - 平均 CPU 使用率 (average_cpu_usage)
///
/// # 性能监控相关概念
/// - CPU 使用率：CPU 在单位时间内处理工作的时间百分比
/// - 内存使用量：当前被占用的物理内存大小
/// - 采样间隔：两次数据采集之间的时间间隔
///
/// 文档参考：
/// - sysinfo 库文档：https://docs.rs/sysinfo/latest/sysinfo/
/// - Rust 时间测量：https://doc.rust-lang.org/std/time/
/// - 系统监控概念：https://en.wikipedia.org/wiki/System_monitor
pub fn collect_data(tx: Sender<CollectorCommandV1>, collector_id: u128) {
    // 创建系统信息监控器，new_all() 监控所有组件
    let mut sys = sysinfo::System::new_all();

    // 初始刷新：获取内存和 CPU 的基准数据
    sys.refresh_memory();
    sys.refresh_cpu_all();

    // 等待 1 秒让 CPU 使用率数据稳定
    // CPU 使用率需要时间间隔才能准确计算
    std::thread::sleep(std::time::Duration::from_secs_f32(1.0));

    // 主循环：持续收集和发送数据
    loop {
        let now = Instant::now(); // 记录开始时间，用于控制采样间隔

        // 刷新系统信息，获取最新的性能数据
        sys.refresh_memory();
        sys.refresh_cpu_all();

        // 收集内存使用情况
        let total_memory = sys.total_memory(); // 总内存量（字节）
        let used_memory = sys.used_memory(); // 已使用内存量（字节）

        // 收集 CPU 使用情况
        let num_cpus = sys.cpus().len(); // CPU 核心数量
        let total_cpu_usage = sys
            .cpus()
            .iter()
            .map(|x| x.cpu_usage()) // 获取每个核心的使用率
            .sum::<f32>(); // 计算所有核心的总使用率
        let average_cpu_usage = total_cpu_usage / num_cpus as f32; // 平均使用率

        // 通过通道发送数据到主线程
        let send_result = tx.send(CollectorCommandV1::SubmitData {
            collector_id,
            total_memory,
            used_memory,
            average_cpu_usage,
        });

        // 处理发送错误（通常是因为通道已关闭）
        if let Err(e) = send_result {
            println!("Error sending data {e:?}");
        }

        // 精确控制采样间隔为 1 秒
        let elapsed_seconds = now.elapsed().as_secs_f32();
        if elapsed_seconds < 1.0 {
            // 如果本次循环耗时少于 1 秒，补充剩余时间
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0 - elapsed_seconds));
        } else {
            // 如果本次循环耗时超过 1 秒，直接等待 1 秒
            // 这种情况通常发生在系统负载很高时
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
        }
    }
}

/// 网络发送函数 - 将编码后的数据发送到中央服务器
///
/// 这个函数负责：
/// 1. 将 CollectorCommandV1 命令编码为二进制格式
/// 2. 建立 TCP 连接到数据收集服务器
/// 3. 发送编码后的数据
///
/// # 参数
/// * `command` - 要发送的数据收集命令
///
/// # 网络协议相关概念
/// - TCP (Transmission Control Protocol): 可靠的、面向连接的传输协议
/// - 数据编码：将结构化数据转换为字节序列以便网络传输
/// - 客户端-服务器架构：收集器作为客户端，服务器接收数据
///
/// # 错误处理
/// 当前使用 unwrap() 简化错误处理，在生产环境中应该：
/// - 处理网络连接失败的情况
/// - 实现重试机制
/// - 记录详细的错误信息
///
/// 文档参考：
/// - Rust TCP 编程：https://doc.rust-lang.org/std/net/struct.TcpStream.html
/// - TCP 协议规范：https://tools.ietf.org/html/rfc793
/// - 网络编程基础：https://en.wikipedia.org/wiki/Network_programming
pub fn send_command(command: CollectorCommandV1) {
    // 使用共享库中的编码函数将命令转换为二进制格式
    // 编码过程包括：序列化、压缩、添加协议头等
    let bytes = shared_data::encode_v1(command);
    println!("Encoded {} bytes", bytes.len());

    // 建立 TCP 连接到数据收集服务器
    // DATA_COLLECTOR_ADDRESS 在 shared_data 中定义为 "127.0.0.1:9004"
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS).unwrap();

    // 将编码后的字节流写入网络连接
    // write_all 确保所有数据都被发送
    stream.write_all(&bytes).unwrap();
}

pub fn send_queue(queue: &mut VecDeque<Vec<u8>>) -> Result<(), CollectorError> {
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;

    let mut buf: Vec<u8> = vec![0u8; 512];
    while let Some(command) = queue.pop_front() {
        if stream.write_all(&command).is_err() {
            queue.push_front(command);
            return Err(CollectorError::UnableToSend);
        }

        let bytes_read = stream
            .read(&mut buf)
            .map_err(|_| CollectorError::UnableToReceive)?;
        if bytes_read == 0 {
            queue.push_front(command);
            return Err(CollectorError::UnableToReceive);
        }

        let ack = decode_response_v1(&buf[0..bytes_read]);
        if ack != CollectorResponseV1::Ack(0) {
            queue.push_front(command);
            return Err(CollectorError::UnableToReceive);
        } else {
            println!("Ack received!");
        }
    }

    Ok(())
}
/// 主函数 - 数据收集器的入口点
///
/// 这个函数实现了生产者-消费者模式：
/// 1. 获取收集器的唯一标识符
/// 2. 创建线程间通信通道
/// 3. 启动数据收集线程（生产者）
/// 4. 主线程负责网络发送（消费者）
///
/// # 线程架构设计
/// - 数据收集线程：专注于系统性能监控，避免网络 I/O 阻塞
/// - 主线程：专注于网络通信，处理连接和发送
/// - 通道通信：线程间安全的数据传递机制
///
/// # 并发编程相关概念
/// - MPSC (Multiple Producer Single Consumer): 多生产者单消费者通道
/// - 线程分离：允许线程独立运行而不阻塞主线程
/// - 数据流水线：数据收集 -> 传输的处理流水线
///
/// # 程序生命周期
/// 1. 初始化和身份识别
/// 2. 启动后台数据收集
/// 3. 持续的网络数据发送
/// 4. 程序运行直到被外部终止
///
/// 文档参考：
/// - Rust 并发编程：https://doc.rust-lang.org/std/thread/
/// - MPSC 通道：https://doc.rust-lang.org/std/sync/mpsc/
/// - 生产者-消费者模式：https://en.wikipedia.org/wiki/Producer%E2%80%93consumer_problem
fn main() {
    // 1. 获取收集器的持久化唯一标识符
    let uuid = get_uuid();

    // 2. 创建 MPSC 通道用于线程间通信
    // tx - 发送端（给数据收集线程使用）
    // rx - 接收端（主线程使用）
    let (tx, rx) = std::sync::mpsc::channel::<CollectorCommandV1>();

    // 3. 启动数据收集线程
    // move 关键字将 uuid 和 tx 的所有权转移给新线程
    // _collector_thread 变量防止线程被立即分离
    let _collector_thread = std::thread::spawn(move || {
        collect_data(tx, uuid); // 在独立线程中持续收集数据
    });

    // 4. 主线程：从通道接收数据并发送到网络
    // 这是一个阻塞循环，持续等待来自数据收集线程的数据
    while let Ok(command) = rx.recv() {
        send_command(command); // 将数据发送到中央服务器
    }

    // 注意：当前程序没有优雅的退出机制
    // 在生产环境中应该添加信号处理来支持优雅关闭
}
