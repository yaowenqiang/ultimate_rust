use once_cell::sync::Lazy;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use std::time::Duration;
use std::{collections::VecDeque, sync::Mutex};

static WORK_QUEUE: Lazy<Mutex<VecDeque<String>>> = Lazy::new(|| Mutex::new(VecDeque::new()));

fn main() {
    // let cpu_count = num_cpus::get();
    let cpu_count = 2;

    let mut threads: Vec<JoinHandle<()>> = Vec::with_capacity(cpu_count);
    let mut broadcast: Vec<Sender<()>> = Vec::with_capacity(cpu_count);

    for cpu in 0..cpu_count {
        let (tx, rx) = mpsc::channel::<()>();
        broadcast.push(tx);
        let thread = std::thread::spawn(move || {
            while rx.recv().is_ok() {
                let mut lock = WORK_QUEUE.lock().unwrap();
                if let Some(work) = lock.pop_front() {
                    std::mem::drop(lock);
                    println!("CPU {cpu} got work: {work}");
                    std::thread::sleep(Duration::from_secs(2));
                    println!("CPU {cpu} finished");
                } else {
                    println!("CPU {cpu} found no work!");
                }
            }
        });
        threads.push(thread);
    }

    loop {
        let send: bool = {
            let mut lock = WORK_QUEUE.lock().unwrap();
            let len = lock.len();
            println!("There are {len} items in the queue");
            if len < 5 {
                lock.push_back("Hello".to_string());
                true
            } else {
                false
            }
        };

        if send {
            broadcast.iter().for_each(|tx| {
                tx.send(()).unwrap();
            })
        }
        std::thread::sleep(Duration::from_secs(1));
    }
    threads.into_iter().for_each(|t| t.join().unwrap());
}
