use authentication::{greet_user, login, read_input, LoginAction, LoginRole};

const MAX_TRIES: i32 = 3;

fn main() {
    let mut tries = 0;

    loop {
        println!("Enter your username:");
        let username = read_input();

        println!("Enter your password:");
        let pwd = read_input();

        // Pattern matching FTW
        match login(&username, &pwd) {
            LoginAction::Granted(role) => {
                match role {
                LoginRole::Admin => println!("{}", greet_user("admin")),
                LoginRole::User => println!("{}", greet_user("user")),
            }
            break;
        },
            LoginAction::Denied => {
                println!("Incorrect credentials\n");

                tries += 1;
                if tries >= MAX_TRIES {
                    println!("Too many attempts!");
                    break;
                }
            },
        }
    }
}
