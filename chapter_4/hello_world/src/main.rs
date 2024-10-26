
use std::{env, process::exit, thread};

fn main() {
    let thread_count = match env::args().nth(1) {
        Some(thread_count) => thread_count,
        None => {
            eprintln!("missing thread count");
            exit(1);
        },
    };

    let thread_count = thread_count.trim();

    let thread_count = match thread_count.parse::<u32>() {
        Ok(thread_count) => thread_count,
        Err(err) => {
            eprintln!("thread count: {err}");
            exit(1); 
        },
    };
    
    let mut handlers = Vec::new();
    for thread in 0..thread_count {
        let handler = thread::spawn(move || {
            println!("Hello from thread {thread} of {thread_count}");
        });
        handlers.push(handler);
    }

    println!("Hello from the main thread");

    for handler in handlers {
        let _ = handler.join();
    }
}
