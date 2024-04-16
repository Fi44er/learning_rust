#![warn(clippy::all, clippy::pedantic)]

mod generator;
use generator::{generate, other_fn};

mod prelude {
    pub const MIN: u8 = 0;
    pub const MAX: u8 = 255;
}

use prelude::*;

fn main() {
    other_fn();
    let random = generate();
    println!("Random: {}", random.value);
    println!("Const: {MAX} - {MIN}");
}

pub fn shared() {
    println!("Shared");
}
