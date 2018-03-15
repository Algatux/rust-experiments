
extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {

    let randomic_number = rand::thread_rng().gen_range(0, 100);

    loop {

        println!("Please input your number.");

        let mut string: String = String::new();

        io::stdin().read_line(&mut string)
            .expect("Failed to read line");

        let string: u32 = match string.trim().parse()  {
            Ok(num) => num,
            Err(e) => {
                println!("Error: {}", e);
                continue
            }
        };

        println!("You inserted: {}", string);
        println!("Generated random number: {}", randomic_number);

        match string.cmp(&randomic_number) {
            Ordering::Less => println!("It's smaller"),
            Ordering::Greater => println!("It's the same!"),
            Ordering::Equal => {
                println!("It's bigger");
                break;
            }
        }

    }


}