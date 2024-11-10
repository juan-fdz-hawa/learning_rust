use std::sync::{self, atomic::AtomicI32};

// Gurantees thread safe access/mutation to this variable
// Rust only provides atomic wraps for the base data types.
static COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    let mut handles = Vec::new();
    for _ in 0..1000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..1_1000 {
                // You mutate the data using atomic operations...
                COUNTER.fetch_add(1, sync::atomic::Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    // To read the data you also need an atomic operation
    println!("{}", COUNTER.load(sync::atomic::Ordering::Relaxed));
}
