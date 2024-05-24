use rand::prelude::Distribution;
use rand::{random, Rng};
use rand::distributions::Standard;

#[derive(Debug)]
enum Animal {
    Cat(String),
    Dog,
    Horse,
    Bird,
}

impl Distribution<Animal> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Animal {
//      implement animal
        match rng.gen_range(0..=3) { // rand 0.8
            0 => Animal::Cat("Dave".parse().unwrap()),
            1 => Animal::Dog,
            2 => Animal::Horse,
            _ => Animal::Bird,
        }
    }
}

fn main() {
    let animal: Animal = random();
    dbg!("{}", animal);
}
