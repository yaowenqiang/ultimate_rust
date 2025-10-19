#[warn(clippy::pedantic)]
#[allow(dead_code)]
fn ignore_me() {}
fn main() {
    let numbers = (0..100).collect::<Vec<i32>>();
    // for i in 0..numbers.len() {
    for item in &numbers {
        // println!("{}", item);
        println!("{item}");
    }
}
