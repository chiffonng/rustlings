# Macros

Rust's macro system is very powerful, but also kind of difficult to wrap your
head around. We're not going to teach you how to write your own fully-featured
macros. Instead, we'll show you how to use and create them.

If you'd like to learn more about writing your own macros, the
[macrokata](https://github.com/tfpk/macrokata) project has a similar style
of exercises to Rustlings, but is all about learning to write Macros.

## Notes

There are two types of macros in Rust: declarative and procedural.

- Declarative macros are the most common and are used to generate code at compile time.
- Procedural macros accept some code as input and produce some code as output. It's similar to higher-order functions or decorators/wrapper functions (`@` in Python).

## Further information

- [Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)
- [The Rust Reference: Macros](https://doc.rust-lang.org/reference/macros.html)
