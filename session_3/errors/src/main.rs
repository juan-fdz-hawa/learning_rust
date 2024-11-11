use serde::Deserialize;
use std::path::Path;
use thiserror::Error;

#[derive(Deserialize)]
struct User {
    user: String,
}

#[derive(Debug, Error)]
enum UsersError {
    #[error("No user fhould")]
    NoUserFound,
    #[error("Another thing")]
    AnotherThing,
}

// Box: Abstract location in the head
// dyn: Figure out size at runtime but it must implement the std::error::Error trait
type GenericType<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn load_users() -> GenericType<Vec<User>> {
    let my_path = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_path)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    Ok(users)
}

fn load_users_2() -> anyhow::Result<Vec<User>> {
    let my_path = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_path)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    anyhow::bail!("Oh no! We can't go on");
    Ok(users)
}

fn load_users_3() -> Result<Vec<User>, UsersError> {
    let my_path = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_path).map_err(|e| UsersError::NoUserFound)?;
    let users: Vec<User> = serde_json::from_str(&raw_text).map_err(|e| UsersError::AnotherThing)?;
    Ok(users)
}

fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("error.txt");
    std::fs::read_to_string(my_file)
}

fn file_uppercase() -> Result<String, std::io::Error> {
    let contents = maybe_read_a_file()?;
    Ok(contents.to_uppercase())
}

fn example() {
    if let Ok(contents) = file_uppercase() {
        // Do something with contents
    }
}

fn main() {
    let my_file = Path::new("error.txt");
    let contents = std::fs::read_to_string(my_file);

    match contents {
        Ok(contents) => println!("{contents}"),
        // Error types must be debug printable
        // Err(e) => println!("Error {e:#?}"),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => println!("File not found error.text"),
            _ => println!("Error {e:#?}"),
        },
    }
}
