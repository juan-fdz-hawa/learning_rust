use std::{sync::mpsc, thread::spawn};

enum Commands {
    SayHello,
    Quit,
}

fn main() {
    let (tx, rx) = mpsc::channel::<Commands>();

    let receiver = spawn(move || {
        // If no message, the thread will park
        while let Ok(command) = rx.recv() {
            match command {
                Commands::SayHello => println!("Hello"),
                Commands::Quit => {
                    println!("Quitting");
                    break;
                }
            }
        }
    });

    for _ in 0..10 {
        tx.send(Commands::SayHello).unwrap();
    }
    tx.send(Commands::Quit).unwrap();

    receiver.join().unwrap();
}
