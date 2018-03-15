
use std::io::Write;
use std::str::FromStr;

fn main() {

    // type can be inferred
    let mut numbers: Vec<u64> = Vec::new();

    for str in std::env::args().skip(1) {
        numbers.push(u64::from_str(&str).expect("error parsing argument"));
    }

    if 0 == numbers.len() {
        writeln!(std::io::stderr(), "No arguments passed").unwrap();
        std::process::exit(1);
    }

    let mut total: u64 = 0;
    for number in numbers {
        total += number;
    }

    println!("The sum is {}", total);

}
