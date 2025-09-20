use std::thread;
fn hello_thread() {
    println!("Hello from thread!");
}
fn main() {
    println!("Hello from main thread!");
    let thread_handle = thread::spawn(|| hello_thread());
    thread_handle.join().unwrap();
}
