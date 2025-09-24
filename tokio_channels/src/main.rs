use std::{sync::mpsc, time::Duration};

enum Command {
    Print(String),
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<Command>();
    let (tx_reply, mut rx_reply) = tokio::sync::mpsc::channel::<String>(20);
    let handle = tokio::runtime::Handle::current();

    std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::Print(value) => {
                    // println!("{:#?}", value);
                    let tx_reply = tx_reply.clone();
                    handle.spawn(async move {
                        tx_reply.send(value).await.unwrap();
                    });
                }
            }
        }
    });

    tokio::spawn(async move {
        while let Some(reply) = rx_reply.recv().await {
            println!("Reply: {}", reply);
        }
    });

    let mut counter = 0;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        tx.send(Command::Print(format!("Hello {counter}"))).unwrap();
        counter += 1;
    }

    let (tx, mut rx) = tokio::sync::broadcast::channel::<String>(10);
    tx.send("Hello!".to_string()).unwrap();
}
