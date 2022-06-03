extern crate core;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
// use std::io::Error;
// use std::num::ParseIntError;

fn main() {
    println!("Guess the number!");

    let random_number: u8 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut user_input: String = String::new();

        let user_input: String = match io::stdin().read_line(&mut user_input) {
            Ok(_) => user_input,
            Err(err) => {
                eprintln!("Read error: {:#}", err.to_string());
                continue;
            }
        };

        let user_input: u8 = match user_input.trim().parse::<u8>() {
            Ok(num) => num,
            Err(err) => {
                eprintln!("Input error: {:#}", err.to_string());
                continue;
            }
        };

        // let user_input: u8 = user_input.unwrap();
        match user_input.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
