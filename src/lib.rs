
pub mod generator {
    extern crate rand;

    use self::rand::Rng;

    pub fn generate_number(a: u32, b: u32) -> u32 {

        rand::thread_rng().gen_range(a, b)
    }
}

pub mod io {
    use std::io;

    pub fn read_number() -> u32 {
        let mut string: String = String::new();

        io::stdin().read_line(&mut string)
            .expect("Failed to read line");

        match string.trim().parse()  {
            Ok(num) => num,
            Err(e) => {
                panic!("Error: {}", e)
            }
        }
    }
}
