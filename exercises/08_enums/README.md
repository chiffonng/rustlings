# Enums

Rust allows you to define types called "enums" which enumerate possible values.
Enums are a feature in many languages, but their capabilities differ in each language. Rust’s enums are most similar to algebraic data types in functional languages, such as F#, OCaml, and Haskell.
Useful in combination with enums is Rust's "pattern matching" facility, which makes it easy to run different code for different values of an enumeration.

## Syntax

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

## Struct vs Enum

- Use structs when you want to model a clear and fixed structure of related properties.
- Use enums when you want to encapsulate different states or types of a variable, particularly when these states are significantly different or when you want to pattern match on them later in your code.

## Further information

- [Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Pattern syntax](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html)
