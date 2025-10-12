use shared_data::{DATA_COLLECTOR_ADDRESS, CollectorCommandV1, decode_v1};
use sqlx::{Pool, Sqlite};
use std::net::SocketAddr;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

pub async fn data_collector(conn: Pool<Sqlite>) -> anyhow::Result<()> {
    let listener = TcpListener::bind(DATA_COLLECTOR_ADDRESS).await?;

    loop {
        let (socket, address) = listener.accept().await?;
        tokio::spawn(new_collection(socket, address, conn.clone()));
    }
}

async fn new_collection(mut socket: TcpStream, address: SocketAddr, conn: Pool<Sqlite>) {
    println!("new collection from {address:?}");
    let mut buf = vec![0u8; 1024];
    loop {
        let n = socket
            .read(&mut buf)
            .await
            .expect("failed to read data from socket");

        if n == 0 {
            println!("No data received - connection closed!");
            return;
        }

        println!(" received {} bytes", n);
        let received_data = decode_v1(&buf[..n]);

        match received_data {
            (
                timestamp,
                CollectorCommandV1::SubmitData {
                    collector_id,
                    total_memory,
                    used_memory,
                    average_cpu_usage,
                },
            ) => {
                let collector_id = uuid::Uuid::from_u128(collector_id);
                let collector_id = collector_id.to_string();
                let result = sqlx::query(
                    "insert into timeseries(collector_id, received, total_memory, used_memory, average_cpu) values(?, ?, ?, ?, ?)"
                ).bind(collector_id)
                    .bind(timestamp)
                    .bind(total_memory as i64)
                    .bind(used_memory as i64)
                    .bind(average_cpu_usage)
                    .execute(&conn)
                    .await;

                if result.is_err() {
                    println!("Error insert into the database {result:?}");
                }
            }
        }
    }
}
