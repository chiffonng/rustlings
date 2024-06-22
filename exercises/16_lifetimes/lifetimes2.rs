// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

// Listing 10-22: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotation-syntax

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// The lifetime of the reference 'result' must be the same as or smaller than the lifetime of the references 'x' (string1) and 'y' (string2), which lasts until the end of main function } and the inner scope } (3rd brace) respectively. So we need to move println!() with 'result' inside the inner scope.
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{}'", result);
    }
}
