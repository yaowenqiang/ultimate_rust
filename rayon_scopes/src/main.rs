fn test() {
    println!("Running tests");
}
fn main() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    // pool.spawn(|| println!("hello from apool thread"));
    // pool.scope(|s| {
    //     for n in 0..20 {
    //         s.spawn(move |_| {
    //             println!("hello from scoped thread {}", n);
    //         })
    //     }
    // });
    //
    // println!("Hello, from the main thread!");

    // pool.scope(|s| {
    //     s.spawn_broadcast(|_scope, context| {
    //         println!("Hello from Broadcast thread {}", context.index());
    //     })
    // });

    pool.join(test, test);
}
