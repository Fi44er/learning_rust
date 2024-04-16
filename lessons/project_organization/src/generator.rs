use crate::prelude::*;
use rand::Rng;

mod random_number;
use random_number::RandomNumber;

pub fn generate() -> RandomNumber {
    super::shared();
    let random_number = rand::thread_rng().gen_range(MIN..=MAX);
    RandomNumber::new(random_number)
}

pub fn other_fn() {
    println!("Other function called");
}
