use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Command {
    Run(Job),
    Quit,
}
fn hi_there() {
    println!("Hi there!");
}

fn main() {
    let (tx, rx) = mpsc::channel::<Job>();
    let handle = std::thread::spawn(move || {
        while let Ok(job) = rx.recv() {
            job();
        }
    });

    let job = || println!("Hello from closure!");

    let job2 = || {
        for i in 1..10 {
            println!("hi there: {}", i);
        }
    };

    tx.send(Box::new(job)).unwrap();
    tx.send(Box::new(job2)).unwrap();
    tx.send(Box::new(hi_there)).unwrap();
    tx.send(Box::new(|| println!("Hi there from box"))).unwrap();

    handle.join().unwrap();
}
