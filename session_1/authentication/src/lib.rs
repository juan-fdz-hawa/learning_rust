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
#[derive(PartialEq, Debug)]
pub enum LoginRole  {
    Admin,
    User,
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied
}

pub fn login(username: &str, password: &str) -> LoginAction {
    if username == "admin" && password == "password" {
        LoginAction::Granted(LoginRole::Admin)
    } else if username == "bob" && password == "password" {
        LoginAction::Granted(LoginRole::User)
    } else {
        LoginAction::Denied
    }
}

pub fn read_input() -> String {
    let mut buffer = String::new();

    stdin()
        .read_line(&mut buffer)
        .expect("Can't read from stdin");

    buffer.trim().to_string()
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
