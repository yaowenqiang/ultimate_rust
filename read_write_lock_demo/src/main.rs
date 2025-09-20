use once_cell::sync::Lazy;
use std::sync::RwLock;

static USERS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(build_users()));

fn build_users() -> Vec<String> {
    vec![String::from("Alice"), String::from("Bob")]
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    std::thread::spawn(|| {
        loop {
            println!("Current user (in a thread)");
            let users = USERS.read().unwrap();
            println!("{users:?}");
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    });

    loop {
        println!("Enter a name to add to the user list!, (or q to quit");
        let input = read_line();
        if input == "q" {
            break;
        } else {
            let mut lock = USERS.write().unwrap();
            lock.push(input.trim().to_string());
        }
    }
}
