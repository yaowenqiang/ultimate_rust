// use console_subscriber;
use std::time::Duration;
use tracing_subscriber::fmt::format::FmtSpan;

#[tracing::instrument]
async fn hello_world() {
    println!("hello world");
    tokio::time::sleep(Duration::from_secs(2)).await;
}

async fn f1() {
    tracing::info!("starting f1");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // console_subscriber::init();
    // let subscriber = tracing_subscriber::FmtSubscriber::new();
    let subscriber = tracing_subscriber::fmt()
        .compact()
        // .json()
        .with_file(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_target(false)
        .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT | FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    tracing::info!("starting up");

    tracing::warn!("Are you sure this is a good idea?");
    tracing::error!("Something went horribly wrong!");

    let _ = tokio::spawn(async move {
        tracing::warn!("something went horribly wrong! inside this thread");
    })
    .await;

    f1().await;
    hello_world().await;
    Ok(())
}
