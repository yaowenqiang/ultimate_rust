use anyhow;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    spawn,
};

// async fn tcp_client() -> anyhow::Result<()> {
// async fn tcp_client() -> anyhow::Result<()> {
//     let mut stream: TcpStream = TcpStream::connect("127.0.0.1:8123").await?;
//     println!("Connected to server");
//     stream.write_all(b"hello world").await?;
//     let mut buffer = vec![0; 1024];
//
//     let bytes_read = stream.read(&mut buffer).await?;
//     println!(
//         "response: {:?}",
//         String::from_utf8_lossy(&buffer[..bytes_read])
//     );
//     stream.shutdown(Shutdown::Both).await?;
//
//     Ok(())
// }

// async fn client_runner() {
//     loop {
//         tokio::time::sleep(Duration::from_secs(1)).await;
//         let _ = tcp_client().await;
//     }
// }

#[tokio::main]
async fn main() {
    // tokio::spawn(client_runner());

    let listener = TcpListener::bind("127.0.0.1:8123").await.unwrap();
    loop {
        let (mut socket, address) = listener.accept().await.unwrap();
        spawn(async move {
            println!("Connection from : {address:?}");
            let mut buf = vec![0u8; 1024];
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");
                if n == 0 {
                    return;
                }

                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}
