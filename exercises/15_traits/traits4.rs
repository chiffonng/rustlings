// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a
// hint.

// Default implementation for a trait
// https://doc.rust-lang.org/stable/book/ch10-02-traits.html#default-implementations
pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// Two parameters have generic types T and U, both of which must implement the Licensed trait. This allows the function to accept any two types that implement Licensed, including SomeSoftware and OtherSoftware.
// https://doc.rust-lang.org/stable/book/ch10-02-traits.html#trait-bound-syntax
fn compare_license_types<T: Licensed, U: Licensed>(software: T, software_two: U) -> bool {
    software.licensing_info() == software_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(other_software, some_software));
    }
}
