pub fn greet_user(name: &str) -> String {
    format!("Hello, {name}!")
}
pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let users = User::get_users();
    if let Some(user) = users.iter().find(|user| user.username == username) {
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

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(PartialEq, Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        User {
            username: username.to_lowercase(),
            password: password.to_string(),
            role: role,
        }
    }

    pub fn get_users() -> [User; 2] {
        [
            Self::new("Bob", "password", LoginRole::User),
            Self::new("Joy", "password", LoginRole::User),
        ]
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
