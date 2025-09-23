#[allow(unused_variables)]
#[allow(dead_code)]
use std::time::Instant;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// fn main() {
//     let now: Instant = Instant::now();
//     let mut line_count = 0;
//
//     if let Ok(lines) = read_lines("test.txt") {
//         lines.for_each(|line| {
//             if let Ok(line) = line {
//                 if !line.trim().is_empty() {
//                     line_count += 1;
//                 }
//             }
//         });
//     }
//     println!(
//         "Read {line_count} lines in {:.3} seconds",
//         now.elapsed().as_secs_f32()
//     );
// }
// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where
//     P: AsRef<Path>,
// {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }

async fn lines_count(filename: String) -> anyhow::Result<usize> {
    let now = Instant::now();
    let mut line_count = 0;
    if let Ok(lines) = read_lines(filename) {
        lines.for_each(|line| {
            if let Ok(line) = line {
                if !line.trim().is_empty() {
                    line_count += 1;
                }
            }
        })
    };
    println!(
        "Read {} lines in {:.3} seconds",
        line_count,
        now.elapsed().as_secs_f32()
    );
    Ok(line_count)
}

async fn async_lines_count(filename: String) -> anyhow::Result<usize> {
    use tokio::fs::File;
    use tokio::io::AsyncBufReadExt;
    use tokio::io::BufReader;

    println!("reading {filename}");

    // let now = Instant::now();
    let mut line_count = 0;
    let file = File::open(filename).await?;

    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        if !line.trim().is_empty() {
            line_count += 1;
        }
    }

    Ok(line_count)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Read file.txt");
    let now: Instant = Instant::now();
    // let (c1, c2) = tokio::join!(
    //     lines_count("file.txt".to_string()),
    //     lines_count("file.txt".to_string()),
    // );

    let (c1, c2) = tokio::join!(
        async_lines_count("file.txt".to_string()),
        async_lines_count("file.txt".to_string()),
    );

    println!("total lines: {}", c1? + c2?);

    println!("in {:.3} seconds", now.elapsed().as_secs_f32());
    Ok(())
}
