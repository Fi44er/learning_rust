#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug)]
struct Car {
    brand: String,
    max_speed: u16,
    max_gas: f32,
    current_gas: f32,
    gas_consumption: f32,
}

impl Car {
    fn new(
        brand: &str,
        max_speed: u16,
        max_gas: f32,
        current_gas: f32,
        gas_consumption: f32,
    ) -> Self {
        Self {
            brand: String::from(brand),
            max_speed,
            max_gas,
            current_gas,
            gas_consumption,
        }
    }

    fn drive(&mut self, distance: f32) {
        let total_gas_consumed = distance * self.gas_per_km();
        if total_gas_consumed > self.current_gas {
            println!("No enough gas!");
        } else {
            println!("Driving!");
            self.current_gas -= total_gas_consumed;
        }
    }

    fn gas_per_km(&self) -> f32 {
        self.gas_consumption / 100.0
    }
}

fn main() {
    let mut my_car = Car {
        brand: String::from("Ford"),
        max_speed: 200,
        max_gas: 50.0,
        current_gas: 50.0,
        gas_consumption: 5.0,
    };

    println!("My car: {my_car:#?}");
    let distance: f32 = 40.0;
    my_car.drive(distance);

    let car1 = Car::new("Volvo", 250, 60.0, 50.5, 3.0);
    println!("Car 1: {car1:#?}");

    let car2 = Car {
        current_gas: 20.0,
        ..car1
    };

    print!("Car 2: {car2:#?}");
}
