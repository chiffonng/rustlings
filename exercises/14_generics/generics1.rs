// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

// https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#in-function-definitions

fn main() {
    // 'milk' is a string slice (&str)
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
