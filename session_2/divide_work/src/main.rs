fn main() {
    // Example of how to use threads to divide your work

    const N_THREADS: usize = 4;

    let to_add: Vec<u32> = (0..5000).collect();
    let mut thread_handles = Vec::new();

    // Chunks use slices underneath so it should be very
    // efficient...
    let chunks = to_add.chunks(N_THREADS);

    for chunk in chunks {
        // Similar to copying the vector
        let my_chunk = chunk.to_owned();
        thread_handles.push(std::thread::spawn(move || my_chunk.iter().sum::<u32>()));
    }

    // Total of each chunk's sum
    let mut sum = 0;
    for h in thread_handles {
        sum += h.join().unwrap()
    }

    println!("The total is {}", sum);
}
