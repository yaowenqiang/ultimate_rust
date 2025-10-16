use memmap2::MmapOptions;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
fn main() {
    let now = std::time::Instant::now();
    let file = File::open("../warandpeace.txt").unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    let buffered_reader = BufReader::new(&mmap[..]);
    println!("Line readed {}", buffered_reader.lines().count());
    println!("time elapsed: {:?}", now.elapsed().as_millis());
}
