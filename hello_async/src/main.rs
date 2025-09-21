use futures::executor::block_on;
use futures::future::join_all;
use futures::join;

fn do_something_sync() {
    println!("Not async!");
}
async fn say_hello() {
    println!("Hello, world!");
    join!(second_function(), say_goodbye());

    let n = double(14).await;
    println!("{n}");

    let futures = vec![double(1), double(2), double(3), double(4)];

    let results = join_all(futures).await;

    println!("{results:?}");

    do_something_sync();
}

async fn second_function() {
    println!("Hello, again!");
}

async fn say_goodbye() {
    println!("goodbye");
}

async fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    block_on(say_hello());
    // let future = say_hello();
    //
    // future.await;
}
