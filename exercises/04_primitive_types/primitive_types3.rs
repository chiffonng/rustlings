// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

// https://doc.rust-lang.org/book/ch03-02-data-types.html

fn main() {
    // let a = [1..=100]; // an array of 100 elements, from 1 to 100
    let a = [1..101]; // an array of 100 elements, from 1 to 100; exclude 101

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed") // raise runtime error
    }
}
