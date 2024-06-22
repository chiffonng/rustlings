// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.

// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#alternatives-to-using-match-with-resultt-e

use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // Use unwrap_or_else() to handle the error case
    let cost = total_cost(pretend_user_input).unwrap_or_else(|e| {
        println!("Error: {}", e);
        0
    });

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?; // Use '?' operator to return early Err(e) from the function that returns Result value

    Ok(qty * cost_per_item + processing_fee)
}
