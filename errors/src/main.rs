use serde::Deserialize;
use std::path::Path;
use thiserror::Error;

fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("my_file.txt");
    std::fs::read_to_string(my_file)
}

fn file_to_uppercase() -> Result<String, std::io::Error> {
    let content = maybe_read_a_file()?;
    Ok(content.to_uppercase())
}

#[derive(Deserialize)]
struct User {
    user: String,
}

#[derive(Debug, Error)]
enum UsersError {
    #[error("No users found")]
    NoUsers,
    #[error("Too many users found")]
    TooManyUsers,
}

type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// fn load_user() -> GenericResult<Vec<User>> {
// fn load_user() -> anyhow::Result<Vec<User>> {
fn load_user() -> Result<Vec<User>, UsersError> {
    let my_path = Path::new("users.json");
    let raw_text = std::fs::read_to_string(&my_path).map_err(|_| UsersError::NoUsers)?;
    let users: Vec<User> = serde_json::from_str(&raw_text).map_err(|_| UsersError::NoUsers)?;
    // anyhow::bail!("Oh no we can't go on!");
    Ok(users)
}

fn main() {
    let _ = file_to_uppercase();
    if let Ok(content) = maybe_read_a_file() {
        println!("{}", content);
    }
    let my_file = Path::new("./myfile.txt");
    let content = std::fs::read_to_string(my_file);
    match content {
        Ok(content) => {
            println!("File content: {}", content);
        }
        Err(err) => match err.kind() {
            // println!("ERROR:{err:#?}");
            std::io::ErrorKind::NotFound => {
                println!("File not found {my_file:#?}");
            }
            _ => {
                println!("Error! {err:#?}");
            }
        },
    }
}
