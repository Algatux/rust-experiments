
extern crate clap;
use clap::{Arg, App};

fn main() {

    let matches = App::new("test command")
        .version("1.0")
        .author("Alessandro Galli <a.galli85@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .get_matches();

    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

}
