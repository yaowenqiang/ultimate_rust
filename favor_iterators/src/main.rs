use rayon::prelude::*;
use std::char::MAX;

struct Row {
    language: String,
    message: String,
}

fn get_rows() -> Vec<Row> {
    vec![
        Row {
            language: "Rust".to_string(),
            message: "".to_string(),
        },
        Row {
            language: "Rust".to_string(),
            message: "".to_string(),
        },
        Row {
            language: "Rust".to_string(),
            message: "".to_string(),
        },
        Row {
            language: "Rust".to_string(),
            message: "".to_string(),
        },
        Row {
            language: "Rust".to_string(),
            message: "".to_string(),
        },
        Row {
            language: "Rust".to_string(),
            message: "".to_string(),
        },
        Row {
            language: "Rust".to_string(),
            message: "".to_string(),
        },
        Row {
            language: "Rust".to_string(),
            message: "".to_string(),
        },
        Row {
            language: "Rust".to_string(),
            message: "".to_string(),
        },
        Row {
            language: "Rust".to_string(),
            message: "".to_string(),
        },
        Row {
            language: "Rust".to_string(),
            message: "".to_string(),
        },
        Row {
            language: "Rust".to_string(),
            message: "".to_string(),
        },
        Row {
            language: "Rust".to_string(),
            message: "".to_string(),
        },
        Row {
            language: "Rust".to_string(),
            message: "".to_string(),
        },
        Row {
            language: "Rust".to_string(),
            message: "".to_string(),
        },
        Row {
            language: "Rust".to_string(),
            message: "".to_string(),
        },
    ]
}
fn main() {
    let now = std::time::Instant::now();
    let rows = get_rows();
    // for row in rows {
    //     if row.language == "French" {
    //         println!("{}", row.message);
    //         break;
    //     }
    // }

    for row in rows.iter() {
        if row.language == "French" {
            println!("{}", row.message);
            break;
        }
    }

    println!("Elapsed {} nanos", now.elapsed().as_nanos());

    let now = std::time::Instant::now();
    rows.iter()
        .filter(|row| row.language == "French")
        .for_each(|row| println!("{}", row.message));
    println!("Elapsed {} nanos", now.elapsed().as_nanos());

    let now = std::time::Instant::now();
    const MAX: u32 = 200000;
    let mut count = 0;
    for n in 2..MAX {
        if is_prime(n) {
            count += 1;
        }
    }

    println!(
        "Found {count} primes in {:.2} seconds",
        now.elapsed().as_secs_f32()
    );

    let now = std::time::Instant::now();

    let count = (2..MAX).filter(|n| is_prime(*n)).count();
    println!(
        "Found {count} primes in {:.2} seconds",
        now.elapsed().as_secs_f32()
    );

    use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
    let now = std::time::Instant::now();
    let count = (2..MAX).into_par_iter().filter(|n| is_prime(*n)).count();
    println!(
        "Found {count} primes in {:.2} seconds",
        now.elapsed().as_secs_f32()
    );

    let mut v = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    v.iter().for_each(|n| println!("{}", n));
    println!("{v:?}");
    // let mut v = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    // v.into_iter().for_each(|n| println!("{}", n));
    // println!("{v:?}");
}

fn is_prime(n: u32) -> bool {
    (2..=n / 2).all(|i| n % i != 0)
}
