// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

#[derive(Debug)]
enum Message<'a> {
    // TODO: define a few types of messages as used below
    Quit(&'a str),
    Echo(&'a str),
    Move(i32,i32),
    ChangeColor(&'a str),
}

fn main() {
    println!("{:?}", Message::Quit("Quiting"));
    println!("{:?}", Message::Echo("Echo"));
    println!("{:?}", Message::Move(5,6));
    println!("{:?}", Message::ChangeColor("green"));
}
