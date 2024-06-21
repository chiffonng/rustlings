# Hashmaps

A _hash map_ allows you to associate a value with a particular key.
You may also know this by the names [_unordered map_ in C++](https://en.cppreference.com/w/cpp/container/unordered_map),
[_dictionary_ in Python](https://docs.python.org/3/tutorial/datastructures.html#dictionaries) or an _associative array_ in other languages.

This is the other data structure that we've been talking about before, when
talking about Vecs.

## Syntax

Create, insert values into a hash map, and get a value from a hash map:

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("{:?}", score);
}
```

Update a value in a hash map:

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    // Add 10 to the value of the key "Blue"; or insert the key with a value of 10 if it doesn't exist
    scores.entry(String::from("Blue")).and_modify(|v| *v += 10).or_insert(10);

    println!("{:?}", scores);
}
```

## Further information

- [Storing Keys with Associated Values in Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
