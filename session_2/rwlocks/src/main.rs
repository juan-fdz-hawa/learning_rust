use once_cell::sync::Lazy;
use std::sync::RwLock;

// Use Lazy init ... we need to do this for non-constant variable types.
static USERS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(build_users()));

fn build_users() -> Vec<String> {
    vec!["Bob".to_string(), "Alice".to_string()]
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn main() {
    std::thread::spawn(|| loop {
        println!("Current users");
        let users = USERS.read().unwrap();
        println!("{users:?}");
        std::thread::sleep(std::time::Duration::from_secs(3));
    });

    loop {
        println!("Enter a name");
        let user_name = read_line();
        if user_name == "q" {
            break;
        }

        let mut lock = USERS.write().unwrap();
        lock.push(user_name);
    }
}
