// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0.clone()); // clone vec0 to keep it separate from vec1 for the below tests

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec; // change to mutable to add '88' to the vector (i.e. borrow the vector)

    vec.push(88);

    vec
}
