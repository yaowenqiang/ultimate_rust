use std::fs::read_to_string;
fn main() {
    let now = std::time::Instant::now();
    let war_and_peace = read_to_string("../warandpeace.txt").unwrap();
    println!("line count: {}", war_and_peace.len());
    println!("time elapsed: {:?}", now.elapsed().as_millis());
}
