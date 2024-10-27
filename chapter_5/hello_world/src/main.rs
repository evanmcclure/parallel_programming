
use std::{env, process::exit};

use async_std::task;
use futures::future::join_all;

#[async_std::main]
async fn main() {
    let thread_count = match env::args().nth(1) {
        Some(thread_count) => thread_count,
        None => {
            eprintln!("missing thread count argument");
            exit(1);
        },
    };

    let thread_count = thread_count.trim();

    let thread_count = match thread_count.parse::<u32>() {
        Ok(thread_count) => thread_count,
        Err(err) => {
            eprintln!("thread count: {err:?}");
            exit(1);
        },
    };

    let mut handlers = Vec::new();
    for thread in 0..thread_count {
        let handle =     task::spawn(async move {
            let thread_count = thread_count;
            println!("Hello from thread {thread} of {thread_count}");
        });

        handlers.push(handle);
    }

    join_all(handlers).await;

}
