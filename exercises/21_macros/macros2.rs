// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.

// https://doc.rust-lang.org/reference/macros-by-example.html#textual-scope
// Since the macro is defined in the same file, it is not necessary to use the `use` statement to bring it into scope.
// However, we have to define the macro before calling it.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
