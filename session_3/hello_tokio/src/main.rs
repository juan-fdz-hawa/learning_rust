use std::thread::yield_now;

use tokio::runtime;

async fn hello() -> u32 {
    println!("Hello Tokio");
    3
}

async fn hello_2() -> u32 {
    println!("Hello 2");
    4
}

fn main_long_way() {
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    // hello is the entry point for the async runtime.
    rt.block_on(hello());
}

async fn ticker() {
    for i in 0..10 {
        println!("tick {i}")
    }
}

async fn ticker_with_yield() {
    for i in 0..10 {
        println!("tick yield {i}");
        yield_now();
    }
}

// Easier way
#[tokio::main]
async fn main() {
    hello().await;

    let results = tokio::join!(hello(), hello_2());
    println!("{results:#?}");

    // Similar to spawining a system thread but not quite ...
    // this might reuse the current task if available (i.e. will run on the same thread
    // depending on config).
    tokio::spawn(ticker()); // Since we are not yielding ... ticker will run from start to finish
    hello().await;

    // This will be concurrent (but not parallel)
    let _ = tokio::join!(tokio::spawn(hello()), tokio::spawn(ticker_with_yield()),);
    println!("Finished")
}
