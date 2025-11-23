use log::warn;

fn main() {
    let guess: u32 = "41".parse().expect("Not a number!");

    #[warn(unused_macros)]
    let a_number = Option::Some(10);

    match guess {
        42 => println!("The answer is: {}", 42),
        x if x > 42 => println!("The answer is greater than 42: {}", x),

        // x => println!("The answer is: {}", x),
        _ => println!("The answer is: {}", guess),
    }
    println!("{}", guess);
}
