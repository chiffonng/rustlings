// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // Add '&' to borrow data instead of taking ownership

    string_uppercase(data); // Remove '&' to pass ownership of data to string_uppercase
}

// Should not take ownership
// Add '&' to parameter to borrow data
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
// Remove the reference & from the parameter and 'data'
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
