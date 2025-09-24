use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. 连接到服务器
    let mut stream = TcpStream::connect("127.0.0.1:8123").await?;
    println!("Connected to server!");

    // 2. 发送数据
    let message = "Hello from Tokio client!";
    stream.write_all(message.as_bytes()).await?;
    println!("Sent: {}", message);

    // 3. 接收响应
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer[..n]);

    println!("Received: {}", response);

    // 4. 关闭连接
    stream.shutdown().await?;
    Ok(())
}
