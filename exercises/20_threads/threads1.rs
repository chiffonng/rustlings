// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.

// https://doc.rust-lang.org/book/ch16-01-threads.html

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        // Start a thread for each iteration of the loop
        // 'move' keyword is used to move ownership of i to the spawned thread
        // closure || {} is the code that will be executed in the spawned thread
        // Each thread will sleep for 250ms
        // Return the elapsed time in milliseconds
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // `handle` is of type `JoinHandle` in the `std::thread` module. A `JoinHandle` represents the handle to a thread, which allows you to wait for the thread to finish by calling `.join()` on it.
        // `.join()` blocks the current thread until the thread represented by the handle terminates. It returns a `Result` type
        // `.unwrap()` is used to extract the value from the `Result`. If the result is `Err`, `.unwrap()` will panic, causing the current thread to panic as well. This is a simple way to handle errors in this example
        results.push(
            handle.join().unwrap(), // Extracts the result of the thread's execution, panicking if the thread had panicked.
        );
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
