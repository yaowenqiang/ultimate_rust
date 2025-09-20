use std::thread;

fn main() {
    const N_THREADS: usize = 10;
    let to_add: Vec<i32> = (0..5000).collect();
    let mut thread_handles = Vec::new();

    let chunks = to_add.chunks(N_THREADS);
    for chunk in chunks {
        let my_chunk: Vec<i32> = chunk.to_owned();
        thread_handles.push(thread::spawn(move || my_chunk.iter().sum::<i32>()));
    }

    let mut sum = 0;
    for handle in thread_handles {
        sum += handle.join().unwrap();
    }

    println!("Total: {}", sum);
}
