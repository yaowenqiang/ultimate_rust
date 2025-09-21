async fn hello() -> u32 {
    println!("hello tokio");
    3
}

async fn hello_tokio() -> u32 {
    println!("hello tokio2");
    4
}

async fn ticker() {
    for i in 0..10 {
        println!("tick {i}");
        tokio::task::yield_now().await;
    }
}

#[tokio::main(flavor = "current_thread")]
// #[tokio::main()]
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

    // hello().await;
    //
    // let results = tokio::join!(hello_tokio(), hello());
    // println!("{results:?}");
    //
    // let (one, two) = results;
    // println!("{one:?} {two:?}");

    // tokio::spawn(ticker());
    // hello().await;

    let _ = tokio::join!(
        tokio::spawn(hello()),
        tokio::spawn(ticker()),
        tokio::spawn(ticker())
    );

    println!("finished main");
}
