
use std::str::FromStr;

fn main() {

    let mut total: u64 = 0;

    for str in std::env::args().skip(1) {
        let number = u64::from_str(&str).expect("error parsing argument");
        total += number;
    }

    println!("The sum is {}", total);

}
