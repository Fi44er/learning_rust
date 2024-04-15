#![warn(clippy::all, clippy::pedantic)]

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n_input = process(&input);

    println!("String: {input} \nNumber: {n_input}");
}

fn process(str: &str) -> u8 {
    str.trim().parse::<u8>().expect("Please enter a number")
}
