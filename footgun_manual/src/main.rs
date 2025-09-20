use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

static COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    let mut handles = Vec::new();

    for _ in 0..1000 {
        let handle = thread::spawn(move || {
            for _ in 0..1_000 {
                COUNTER.fetch_add(1, Relaxed);
            }
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    println!("{}", COUNTER.load(std::sync::atomic::Ordering::Relaxed));
}
