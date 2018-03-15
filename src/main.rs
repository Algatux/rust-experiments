
extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {

    let randomic_number = rand::thread_rng().gen_range(0, 100);

    println!("Generated random number: {}", randomic_number);

    println!("Please input your number.");

    let mut string: String = String::new();

    io::stdin().read_line(&mut string)
        .expect("Failed to read line");

    println!("You inserted: {}", string);

    let string: u32 = string.trim().parse().expect("Please type a number");

    match string.cmp(&randomic_number) {
        Ordering::Less => println!("It's smaller"),
        Ordering::Equal => println!("It's the same!"),
        Ordering::Greater => println!("It's bigger"),
    }

}