use authentication::{LoginAction, LoginRole, greet_user, login, read_line};
fn main() {
    let mut tries = 0;
    loop {
        println!("Please enter your username:");
        let username = read_line();
        println!("Enter your password:");
        let password = read_line();
        match login(&username, &password) {
            Some(LoginAction::Granted(LoginRole::Admin)) => {
                println!("Admin1");
            }
            Some(LoginAction::Granted(role)) => {
                match role {
                    LoginRole::Admin => {
                        println!("Admin2");
                    }
                    LoginRole::User => {
                        println!("User");
                    }
                }
                break;
            }
            Some(LoginAction::Denied) => {
                println!("You are not granted!");
            }
            None => {
                println!("use does not exists!");
            }
        }
    }
}
