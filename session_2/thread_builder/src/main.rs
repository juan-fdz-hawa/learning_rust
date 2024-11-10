use std::thread;

fn my_thread() {
    println!("Hello from {}", thread::current().name().unwrap())
}

fn main() {
    // Alternative to spawn for building threads

    let handler = thread::Builder::new()
        .name("Thread 1".to_string())
        .stack_size(std::mem::size_of::<usize>() * 40)
        .spawn(my_thread);

    println!("Hello, world!");
}
