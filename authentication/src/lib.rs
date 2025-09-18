use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password.as_bytes());
    format!("{:x}", hasher.finalize())
}
pub fn greet_user(name: &str) -> String {
    format!("Hello, {name}!")
}
pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let password = hash_password(password);
    let users = User::get_users();
    if let Some(user) = users.get(&username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    } else {
        return None;
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        User {
            username: username.to_lowercase(),
            password: hash_password(password),
            role,
        }
    }

    // fn get_admin_users() {
    //     let users = Self::get_users()
    //         .into_iter()
    //         .filter(|user| user.role == LoginRole::Admin)
    //         // .map(|u| u.username)
    //         .collect::<Vec<User>>();
    // }

    // pub fn get_users() -> Vec<User> {
    // vec![
    //     Self::new("Bob", "password", LoginRole::User),
    //     Self::new("Joy", "password", LoginRole::User),
    // ]
    // }
    pub fn get_default_users() -> HashMap<String, User> {
        let mut users = HashMap::new();
        users.insert(
            "admin".to_string(),
            User::new("admin", "password", LoginRole::Admin),
        );
        users.insert(
            "bob".to_string(),
            User::new("bob", "password", LoginRole::User),
        );
        users
    }

    pub fn get_users() -> HashMap<String, User> {
        let users_path = std::path::Path::new("users.json");
        if users_path.exists() {
            let users_json = std::fs::read_to_string(users_path).unwrap();
            let users = serde_json::from_str(&users_json).unwrap();
            users
        } else {
            let mut users = Self::get_default_users();
            let users_json = serde_json::to_string(&users).unwrap();
            std::fs::write(users_path, users_json).unwrap();
            users
        }
    }
}
pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Stdin not working");
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!(greet_user("Greet User"), "Hello, Greet User!");
    }

    #[test]
    fn test_login() {
        assert_eq!(
            login("admin", "password"),
            Some(LoginAction::Granted(LoginRole::Admin))
        );
        assert_eq!(
            login("bob", "password"),
            Some(LoginAction::Granted(LoginRole::User))
        );
        assert_eq!(login("jack", "password"), Some(LoginAction::Denied));
    }
}
