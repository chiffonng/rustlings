// enums1.rs
//
// No hints this time! ;)

// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html

#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
