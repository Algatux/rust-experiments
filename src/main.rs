
mod lib;

use std::cmp::Ordering;

use lib::generator as g;
use lib::io as input;

fn main() {

    let randomic_number = g::generate_number(1, 10);

    loop {

        println!("Please input your number.");

        let string: u32 = input::read_number();

        println!("You inserted: {}", string);
        println!("Generated random number: {}", randomic_number);

        match string.cmp(&randomic_number) {
            Ordering::Less => println!("It's smaller"),
            Ordering::Greater => println!("It's bigger"),
            Ordering::Equal => {
                println!("It's the same!");
                break;
            }
        }

    }
}