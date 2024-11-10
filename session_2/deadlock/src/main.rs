use std::sync::Mutex;

static MY_SHARED: Mutex<i32> = Mutex::new(0);

fn main() {
    // Get lock .... OK
    let lock = MY_SHARED.lock().unwrap();
    // Try to get the lock, this will cause a deadlock because locks
    // are freed at the end of the scope and the first lock will never
    // get freed.
    let lock = MY_SHARED.lock().unwrap();

    {
        // This won't cause a deadlock
        let lock = MY_SHARED.lock().unwrap();
    }

    // Another option is to use a try-lock
    if let Ok(_lock) = MY_SHARED.try_lock() {
        println!("We have the lock");
    } else {
        println!("No lock");
    }
}
