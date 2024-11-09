use authentication::get_users;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // List all users
    List,
    // Adds an user
    Add {
        // The user's login name
        username: String,
        // The user's login password
        password: String,
        // Optional - mark as an admin
        admin: Option<bool>,
    },
}

fn list_users() {
    println!("{:<20}{:<20}", "Username", "Password");
    println!("{:-<40}", "");

    get_users()
        .iter()
        .for_each(|usr| println!("{:<20}{:20?}", usr.username, usr.role));
}

fn add_user(username: String, password: String, is_admin: bool) {}

fn main() {
    let cli = Args::parse();

    match cli.command {
        Some(Commands::List) => {
            list_users();
        }
        Some(Commands::Add {
            username,
            password,
            admin,
        }) => add_user(username, password, admin.unwrap_or_default()),
        None => {
            println!("Run with --help for instructions")
        }
    }
}
