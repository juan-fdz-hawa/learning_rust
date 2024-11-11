use std::{thread::sleep, time::Duration};

async fn hello_delay(task: u64, time: u64) {
    println!("Task {task} has started");
    let _ =
        tokio::task::spawn_blocking(move || std::thread::sleep(Duration::from_millis(time))).await;
    // std::thread::sleep(Duration::from_millis(time));
    // tokio::time::sleep(Duration::from_millis(time)).await;
    println!("Task {task} has finished");
}

#[tokio::main]
async fn main() {
    tokio::join!(
        hello_delay(1, 500),
        hello_delay(1, 1500),
        hello_delay(1, 2500),
    );
}
