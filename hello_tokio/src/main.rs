async fn hello() {
    println!("hello tokio");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // let rt = runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap();

    // let rt = runtime::Builder::new_multi_thread()
    //     .enable_all()
    //     .worker_threads(4)
    //     .build()
    //     .unwrap();
    //
    // rt.block_on(hello());

    hello().await;
}
