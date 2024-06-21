// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

// https://doc.rust-lang.org/std/primitive.array.html

fn main() {
    let a: [i32; 100] = [0; 100]; // 0 repeated 100 times

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed") // raise runtime error
    }
}
