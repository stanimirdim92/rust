extern crate core;

use std::io;
use std::io::BufRead;

fn main() {
    println!("Enter a number: ");

    let mut number: String = String::new();

    let number: String = match io::stdin().lock().read_line(&mut number) {
        Ok(_) => number,
        Err(err) => {
            return eprintln!("Read error: {:#}", err.to_string());
        }
    };

    let number: u64 = match number.trim().parse::<u64>() {
        Ok(number) => number,
        Err(err) => {
            return eprintln!("Not a number! Error: {:#}", err.to_string());
        }
    };

    println!("{}", factorial(number));
}

fn factorial(num: u64) -> u64 {
    if num <= 1 {
        return 1;
    }

    num * factorial(num - 1)
}
