
use std::io;

fn main() {

    let mut string: String = String::new();

    println!("please enter a string");

    io::stdin().read_line(&mut string).expect("failed to read from stdIn");

    println!("You wrote: {}", string);
}