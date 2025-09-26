use async_recursion::*;
use std::pin::Pin;
#[async_recursion]
async fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1).await + fibonacci(n - 2).await,
    }
}

async fn one() {
    println!("one");
}

async fn two() {
    println!("two");
}

async fn call_one_or_two(n: u32) -> Pin<Box<dyn Future<Output = ()>>> {
    match n {
        1 => Box::pin(one()),
        2 => Box::pin(two()),
        _ => panic!("Invalid choice"),
    }
}

#[tokio::main]
async fn main() {
    println!("fibonacci(10): {}", fibonacci(10).await);

    let mut future = async {
        println!("Hello world");
    };
    tokio::pin!(future);
    (&mut future).await;
    call_one_or_two(1).await;
    call_one_or_two(2).await;
}
