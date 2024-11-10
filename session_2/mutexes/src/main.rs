use std::{sync::Mutex, thread::spawn};

static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

fn main() {
    let mut handles = Vec::new();
    for _ in 1..10 {
        let handle = spawn(|| {
            // When the lock leaves the scope that's when it becomes unlock
            let mut lock = NUMBERS.lock().unwrap();
            // The 'lock' becomes the vector
            lock.push(1);
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|th| th.join().unwrap());

    let lock = NUMBERS.lock();
    println!("{:#?}", lock);
}
