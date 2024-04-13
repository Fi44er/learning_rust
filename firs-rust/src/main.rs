use std::io;

const C: f32 = 32.0;

fn c_to_f(c: f32) -> f32 {
    (c * (9.0 / 5.0)) + C
}

fn f_to_c(f: f32) -> f32 {
    (f - C) * (5.0 / 9.0)
}

fn convert(temperature: f32, input: u8) -> Option<f32> {
    match input {
        1 => Some(c_to_f(temperature)),
        2 => Some(f_to_c(temperature)),
        _ => None,
    }
}

fn main() {
    println!("Temperature Converter. \n (1) Celsius to Fahrenheit \n (2) Fahrenheit to Celsius");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n_input = input.trim().parse::<u8>().expect("Please type a number!");

    println!("Enter temperature: ");

    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature).unwrap();
    let n_temperature = temperature
        .trim()
        .parse::<f32>()
        .expect("Please type a number!");

    match convert(n_temperature, n_input) {
        Some(result) => println!("Result: {}", result),
        None => println!("Invalid input!"),
    };
}
