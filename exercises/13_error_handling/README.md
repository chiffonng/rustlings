# Error handling

Most errors aren’t serious enough to require the program to stop entirely.
Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to.
For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.

## Syntax

Enum Result<T, E> is used to return either a value of type T or an error of type E. It is similar to Option<T> but with an error type, or in Python, it is similar to returning data and raise an exception in case of an error.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `match` keyword is used to handle the result of a function that returns a Result type. The `match` keyword is similar to a `switch` statement in other languages.

```rust
fn read_file(file_name: &str) -> Result<String, io::Error> {
    let file = File::open(file_name);
    let mut contents = String::new();
    match file {
        Ok(mut file) => {
            file.read_to_string(&mut contents)?;
            Ok(contents)
        }
        Err(e) => Err(e),
    }
}
```

As a shortcut, the `?` operator can be used to return the error from the current function. The `?` operator can only be used on a `Result` in a function that returns `Result`, and you can use the `?` operator on an `Option` in a function that returns `Option`, but can’t mix and match

```rust
fn read_file(file_name: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

Boxing errors is a way to return multiple error types from a function; the underlying error type is only known at runtime and not statically determined at compile time. The `Box` type is used to store the error type on the heap, and the `?` operator is used to return the error from the current function.

```rust
fn read_file(file_name: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(file_name)?;
    let mut contents = String::new();
    BufReader::new(file).read_to_string(&mut contents)?;
    Ok(contents)
}
```

However, boxing errors are not recommended because it can hide the error type and make it difficult to understand the error. It is better to use a custom error type that implements the `Error` trait.

```rust
use std::fmt::{self, Display, Formatter};
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
enum CustomError {
    Io(io::Error),
    ParseInt(ParseIntError),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            CustomError::Io(ref err) => write!(f, "IO error: {}", err),
            CustomError::ParseInt(ref err) => write!(f, "ParseInt error: {}", err),
        }
    }
}
```

## Further information

- [Error Handling](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
- [Boxing errors](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html)
- [Wrapping errors](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/wrap_error.html)

```

```

```

```

```

```
