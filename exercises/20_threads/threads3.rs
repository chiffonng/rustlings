// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

// https://doc.rust-lang.org/book/ch16-02-message-passing.html#creating-multiple-producers-by-cloning-the-transmitter

use std::sync::mpsc; // multiple producer, single consumer: a channel can have multiple sending ends that produce values, but only one receiving end that consumes those values
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

/// Send the first half of the queue first, then the second half
/// The spawned thread will send multiple messages and pause for a second between each message.
fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    // Sender does NOT implement Copy trait, so it cannot be implicitly copied. We need to clone the Sender for two threads
    // Clone the Sender, so that we can send it to the first spawned thread
    let tx0 = tx.clone();
    thread::spawn(move || {
        for val in q.first_half {
            println!("sending {:?}", val);
            tx0.send(val).unwrap(); // Sender::send() returns a Result<T, E> type. If the receiving end of the channel has been dropped, send() will return an error. In this case, we use unwrap() to panic if an error occurs.
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Use the original Sender to send the second half
    thread::spawn(move || {
        for val in q.second_half {
            println!("sending {:?}", val);
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

#[test]
fn main() {
    // Create a new channel to send `u32` values
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
