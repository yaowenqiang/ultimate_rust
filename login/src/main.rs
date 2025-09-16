use authentication::{greet_user, read_line, login};
fn main() {
    let mut tries = 0;
    loop {
        println!("Please enter your username:");
        let username = read_line();
        println!("Enter your password:");
        let password = read_line();
        if login(&username, &password) {
            println!("Successfully logged in!");
            break;
        } else {
            println!("Failed to log in!");
            tries += 1;
            if tries >= 3 {
                println!("Too many failed logins!");
                break;
            }
        }
    }
}
