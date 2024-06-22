// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

// https://doc.rust-lang.org/rust-by-example/generics/bounds.html

// Wrapper is a trait object that can store any type using generic type T
struct Wrapper<T> {
    value: T,
}

// T uses the trait Wrapper above, so it requires T to be bound to the Wrapper trait. That means T must implement Wrapper 'impl<T> Wrapper<T>'
impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
