# Strings

Rust has two string types, a string slice (`&str`) and an owned string (`String`).
We're not going to dictate when you should use which one, but we'll show you how
to identify and create them, as well as use them.

## String slice VS owned string

In Rust, understanding the difference between `String` and string slices (`&str`) is crucial for managing text data effectively. Here's a detailed comparison table to highlight the differences between these two types:

| **Aspect**            | **String**                                                                           | **&str** (String Slice)                                                                                                                    |
| --------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------ |
| **Type**              | Owned, growable string                                                               | Borrowed reference to a string                                                                                                             |
| **Storage**           | Heap (dynamically allocated)                                                         | Typically inlined or statically allocated; points to some memory region holding string data                                                |
| **Mutability**        | Mutable: can be modified, grown, and shrunk                                          | Immutable by default                                                                                                                       |
| **Creation**          | Created with `String::new()`, `String::from("literal")`, or `"literal".to_string()`  | Created by directly referencing a string literal: `let s: &str = "literal";` or as a borrow from a `String`: `let s: &str = &some_string;` |
| **Common Use Cases**  | Used when you need to modify or own the data                                         | Used for viewing or referencing string data without ownership concerns                                                                     |
| **Lifetimes**         | Managed automatically, scope-based                                                   | Must be explicitly considered in function signatures and structs if they outlive their source                                              |
| **Performance**       | Slightly slower due to allocations and resizing                                      | Faster access, no allocation needed                                                                                                        |
| **Methods Available** | All string methods available plus methods for modification and capacity management   | Limited to methods that do not modify the data                                                                                             |
| **Functionality**     | Can be passed to functions expecting `&str` with a reference (`&`)                   | Cannot be directly converted to `String` without creating a new owned instance                                                             |
| **Example Creation**  | `let mut s = String::from("hello"); s.push('w');`                                    | `let s = "hello";`                                                                                                                         |
| **Cloning/Copying**   | Requires explicit cloning (`s.clone()`) to duplicate because it owns its data        | Can be copied with simple assignment, as it is just a reference                                                                            |
| **Memory Management** | Programmer must manage growth and size, but Rust handles allocation and deallocation | No memory management required by the programmer, as it does not own data                                                                   |

## Further information

- [Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
