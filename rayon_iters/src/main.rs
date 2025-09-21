use rayon::prelude::*;
use std::time::Instant;
fn is_prime(n: usize) -> bool {
    (2..=n / 2).into_par_iter().all(|x| n % x != 0)
}
fn main() {
    let numbers: Vec<usize> = (0..1_000).collect();
    let sum = numbers.par_iter().sum::<usize>();
    println!("Sum: {}", sum);

    let now: Instant = Instant::now();
    let mut primes = numbers
        .par_iter()
        .filter(|&x| is_prime(*x))
        .collect::<Vec<_>>();
    primes.par_sort_unstable();
    let elapsed = now.elapsed().as_secs_f32();
    println!("Found {} primes in: {:?} seconds.", primes.len(), elapsed);
}
