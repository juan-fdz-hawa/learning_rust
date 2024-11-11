use futures::executor::block_on;
use futures::join;

async fn say_hello() {
    println!("Hello");
    // When you await, the current function and the next function
    // is queued.
    join!(second_fn(), third_fn());
}

async fn second_fn() {
    println!("Second fn")
}

async fn third_fn() {
    println!("Say goodbye");
}

fn main() {
    // async is syntax sugar for turning your function into a future.
    // A future represents a UoW that will be executed in the future.
    let future = say_hello();

    // You can not run an async future without a Runtime so this won't work
    // future.await;

    // Give control to the Async Runtime
    block_on(say_hello());

    println!("Hello, world!");
}
