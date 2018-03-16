extern crate rand;

use std::{thread, time};
use rand::Rng;

#[derive(Debug)]
struct Consumer {
    id: u32,
    queue: String,
    assist: Vec<String>,
    active: bool
}

impl Consumer {

    pub fn identifier(&self) -> u32 { self.id }

}

#[derive(Debug)]
struct ConsumerPool {
    threads: u8,
    threads_pool: Vec<Consumer>
}

impl ConsumerPool {

    pub fn size(&self) -> u8 {
        self.threads
    }

    pub fn spawn(&mut self) -> &Consumer {
        self.threads = self.threads.checked_add(1).unwrap();
        self.threads_pool.push(Consumer{
            id: rand::thread_rng().gen_range(1,9999),
            queue: "queue".to_string(),
            assist: vec!["queue2".to_string()],
            active: false
        });

        self.threads_pool.last().unwrap()
    }

    pub fn kill(&mut self) -> Option<Consumer> {
        if self.threads > 0 {
            self.threads = self.threads.checked_sub(1).unwrap();
        }

        self.threads_pool.pop()
    }
}

fn main() {
    let mut pool = ConsumerPool { threads: 0, threads_pool : vec![] };

    println!("Starting pool {:?}", pool);

    loop {

        if is_there_new_message() {
            println!("New message incoming ... Creating new consumer: {}", pool.spawn().identifier());
        } else {
            println!("No messages incoming ... ");
            match pool.kill() {
                Some(c) => println!("Killed consumer: {}", c.identifier()),
                None => println!("Poll already empty")
            }
        }

        println!("Actual pool size {:?}", pool);

        thread::sleep(time::Duration::from_secs(1));
    }
}

fn is_there_new_message() -> bool {
    rand::thread_rng().gen_range(1,10) % 2 == 0
}