// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

// https://doc.rust-lang.org/book/ch16-01-threads.html#using-move-closures-with-threads

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // Mutex ensures that only one thread can access the inner data (jobs_completed) at a time. When a thread wants to read or write the shared data, it must first acquire the lock. This prevents data races and ensures that the data is consistent.
    // Arc (atomic reference counter) allows multiple threads to share ownership of the same data in a thread-safe way
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));

    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // Before any modification, a thread must acquire the lock, which returns MutexGuard. MutexGuard is a smart pointer that implements Deref and Drop traits. Deref allows you to access the inner data, and Drop releases the lock when the MutexGuard goes out of scope.
            let mut job_status = status_shared.lock().unwrap();
            job_status.jobs_completed += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Similarly, allow the thread to access the shared data
    let jobs_completed = status.lock().unwrap().jobs_completed;
    println!("Jobs completed: {}", jobs_completed);
}
