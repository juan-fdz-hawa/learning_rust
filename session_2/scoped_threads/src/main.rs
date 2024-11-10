use std::thread;

fn main() {
    // Thread handles
    const N_THREADS: usize = 8;

    let to_add: Vec<u32> = (0..5000).collect();

    let chunks = to_add.chunks(N_THREADS);

    // Data lifetime is restricted to the specified scope, so this
    // simplifies the code quite a bit because we dont' have to worry
    // about moving data into Threads since the borrow manager can determine
    // that the data won't live outside the scope.
    let sum = thread::scope(|sh| {
        let mut thread_handles = Vec::new();
        for chunk in chunks {
            let thread_handle = sh.spawn(move || chunk.iter().sum::<u32>());
            thread_handles.push(thread_handle);
        }

        thread_handles
            .into_iter()
            .map(|th| th.join().unwrap())
            .sum::<u32>()
    });

    println!("The total is {}", sum);
}
