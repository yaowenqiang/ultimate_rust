fn parkable_thread(n: u32) {
    loop {
        std::thread::park();
        println!("Thread {n} is unparked, briefly");
    }
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut threads = Vec::new();
    for i in 0..10 {
        let thread = std::thread::spawn(move || {
            parkable_thread(i);
        });
        threads.push(thread);
    }

    loop {
        println!("Thread to unpark:");
        let input = read_line();
        if input == "q" {
            break;
        } else {
            if let Ok(number) = input.trim().parse::<usize>() {
                if number < 10 {
                    threads[number].thread().unpark();
                }
            }
        }
    }
}
