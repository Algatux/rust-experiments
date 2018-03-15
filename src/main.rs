
extern crate rand;

use rand::Rng;

fn main() {

    let randomic_number = rand::thread_rng().gen_range(0, 100);

    println!("Generated random number: {}", randomic_number);

}