use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::stdin, path::Path};

// In rust everything is private by default ...
// if we want this to be accessible outside this module we
// need to make it public.
pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

// Use derive to tell Rust to automatically implement some traits
// here Rust is "teaching" the enum type how to equate and how to
// be debuggable.
#[derive(PartialEq, Debug, Clone, Copy, Deserialize, Serialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

pub fn read_input() -> String {
    let mut buffer = String::new();

    stdin()
        .read_line(&mut buffer)
        .expect("Can't read from stdin");

    buffer.trim().to_string()
}

// Struct fields are private by default ...
// Deserialize, Serialize ... sick macros
#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    // 'Constructor'
    pub fn new(username: &str, password: &str, role: LoginRole) -> Self {
        Self {
            username: username.to_lowercase(),
            password: password.to_string(),
            role,
        }
    }
}

fn get_users() -> Vec<User> {
    vec![
        User::new("admin", "password", LoginRole::Admin),
        User::new("bob", "password", LoginRole::User),
    ]
}

pub fn login(username: &str, password: &str) -> LoginAction {
    let users = get_users();

    if let Some(user) = users
        .iter()
        .find(|usr| usr.username == username && usr.password == password)
    {
        return LoginAction::Granted(user.role);
    }
    LoginAction::Denied
}

fn get_admin_usernames() -> Vec<String> {
    // into_iter takes ownership of all elements of the vector
    get_users()
        .into_iter()
        .filter(|u| u.role == LoginRole::Admin)
        .map(|u| u.username)
        .collect()
}

fn get_users_map() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    let mut hash_map = HashMap::new();

    if users_path.exists() {
        let users_json = std::fs::read_to_string(users_path).unwrap();
        let users: Vec<User> = serde_json::from_str(&users_json).unwrap();
        hash_map.insert(users[0].username.clone(), users[0].clone());
        hash_map.insert(users[1].username.clone(), users[1].clone());
    } else {
        let users = get_users();
        let users_json = serde_json::to_string(&users).expect("Could not serialize JSON");
        std::fs::write(users_path, users_json).unwrap();
    }
    hash_map
}

// #[cfg(..)] is a compiler directive
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("Hello juan", greet_user("juan"));
    }

    #[test]
    fn test_login() {
        assert_eq!(
            login("admin", "password"),
            LoginAction::Granted(LoginRole::Admin),
        );
        assert_eq!(
            login("bob", "password"),
            LoginAction::Granted(LoginRole::User),
        );
        assert_eq!(login("admin", "password2"), LoginAction::Denied,);
    }
}
