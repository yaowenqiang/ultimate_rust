use std::time::Duration;
use tokio::select;
use tokio::sync::{broadcast, mpsc};

async fn do_work() {
    tokio::time::sleep(Duration::from_secs(2)).await;
}

async fn receiver(mut rx: mpsc::Receiver<u32>, mut broadcast_rx: broadcast::Receiver<u32>) {
    loop {
        tokio::select! {
            Some(n) = rx.recv() => {
                println!("received message {} on the mpsc channel", n);
            },
            Ok(n) = broadcast_rx.recv() => {
                println!("received message {} on the broadcast channel", n);
            }
        }
    }
}

async fn timeout(seconds: f32) {
    tokio::time::sleep(Duration::from_secs_f32(seconds)).await;
}

#[tokio::main]
async fn main() {
    select! {
        _ = do_work() => println!("do work finished first"),
        _ = timeout(1.0) => println!("timeout finished first"),
    }

    let (tx, mut rx) = mpsc::channel::<u32>(1);

    let (broadcast_tx, mut broadcast_rx) = broadcast::channel::<u32>(1);

    tokio::spawn(receiver(rx, broadcast_rx));

    for count in 0..10 {
        if count % 2 == 0 {
            tx.send(count).await.unwrap();
        } else {
            broadcast_tx.send(count).unwrap();
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
