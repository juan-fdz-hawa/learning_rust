fn hello_thread() {
    println!("Hello from thread")
}

fn hello_thread_2(n: i32) {
    println!("Hello from thread {n}")
}

fn do_math(i: u32) -> u32 {
    i * i
}

fn main() {
    // All programs have at least one thread (the main thread)
    println!("Hello, from the main thread");

    // A thread handle is an interaction point to the thread we
    // just created
    let thread_handle = std::thread::spawn(hello_thread);

    // Wait for the newly created thread to finish before continuin execution.
    thread_handle.join().unwrap();

    let mut thread_handles = Vec::new();
    for i in 0..5 {
        // move: the closure only last as long as the for loop,
        // so to avoid memory errors we have to move it into the thread
        let thread_handle = std::thread::spawn(move || hello_thread_2(i));
        thread_handles.push(thread_handle);
    }

    // Wait for every single thread
    thread_handles.into_iter().for_each(|th| th.join().unwrap());

    // Getting data out of Threads
    let mut mut_thread_handles = Vec::new();
    for i in 0..10 {
        // move: the closure only last as long as the for loop,
        // so to avoid memory errors we have to move it into the thread
        let thread_handle = std::thread::spawn(move || do_math(i));
        mut_thread_handles.push(thread_handle);
    }
    mut_thread_handles
        .into_iter()
        .for_each(|th| println!("{}", th.join().unwrap()));
}
