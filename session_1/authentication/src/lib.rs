use std::io::stdin;

// In rust everything is private by default ...
// if we want this to be accessible outside this module we
// need to make it public.
pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

// Use derive to tell Rust to automatically implement some traits
// here Rust is "teaching" the enum type how to equate and how to 
// be debuggable.
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum LoginRole  {
    Admin,
    User,
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied
}

pub fn read_input() -> String {
    let mut buffer = String::new();

    stdin()
        .read_line(&mut buffer)
        .expect("Can't read from stdin");

    buffer.trim().to_string()
}


// Struct fields are private by default ...
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
            role
        }
    }
}

fn get_users() -> [User; 2] {
    [
        User::new("admin", "password", LoginRole::Admin),
        User::new("bob", "password", LoginRole::User),
    ]
}

pub fn login(username: &str, password: &str) -> LoginAction {
    let users = get_users();

    if let Some(user) = users
        .iter()
        .find(|usr| usr.username == username && usr.password == password) {
            return LoginAction::Granted(user.role)
    } 

        LoginAction::Denied
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
        assert_eq!(login("admin", "password"), LoginAction::Granted(LoginRole::Admin));
        assert_eq!(login("bob", "password"), LoginAction::Granted(LoginRole::User));
        assert_eq!(login("admin", "password2"), LoginAction::Denied);
    }
}
