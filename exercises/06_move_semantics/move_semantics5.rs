// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references

/*
#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    let z = &mut x;
    *y += 100;
    *z += 1000;
    assert_eq!(x, 1200);
}
The attempt to create let z = &mut x; immediately after let y = &mut x; violates Rust’s rule that you can have only one mutable reference to a particular piece of data in a particular scope. This rule is enforced to prevent data races. See https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules for more information.
 */
 
#[test]
fn main() {
    let mut x = 100;

    // creates a mutable reference to x. During the lifetime of y, no other mutable references to x can be created.
    let y = &mut x; 
    *y += 100; // dereference y (with *) and add 100 to x

    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
