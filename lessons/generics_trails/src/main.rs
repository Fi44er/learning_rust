#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug)]
struct Employee<T, U> {
    age: T,
    salary: U,
    tax: U,
}
impl<T, U: std::marker::Copy + std::ops::Sub<Output = U> + std::ops::Mul<Output = U>>
    Employee<T, U>
{
    fn salary_after_tax(&self) -> U {
        self.salary - (self.salary * self.tax)
    }
}

trait Drive {
    fn can_drive(&self) -> bool;
}

struct Car {
    gas: u32,
}
impl Drive for Car {
    fn can_drive(&self) -> bool {
        self.gas > 0
    }
}

struct ElectroCar {
    battery_change: u32,
}

impl Drive for ElectroCar {
    fn can_drive(&self) -> bool {
        self.battery_change > 0
    }
}

fn car_info<T: Drive>(car: &T) {
    println!("Can drive {}", car.can_drive())
}

fn main() {
    let car = Car { gas: 10 };
    let _electro_car = ElectroCar { battery_change: 50 };
    car_info(&car);

    let _employee = Employee {
        age: 12,
        salary: 5000.0,
        tax: 0.3,
    };

    let salary_after_tax = _employee.salary_after_tax();
    println!("Salary after tax: {}", salary_after_tax);

    let nums1: Vec<i32> = vec![1, 2, 3, 4];
    let nums2: Vec<i8> = vec![1, 2, 3, 4];
    let _result1 = sum(&nums1);
    let _result2 = sum(&nums2);
}

// fn sum<T: std::marker::Copy + std::ops::Add<Output = T>>(numbers: &[T]) -> T {
//     numbers.iter().copied().reduce(|acc, n| acc + n).unwrap()
// }

// fn sum<T>(numbers: &[T]) -> T
// where
//     T: std::marker::Copy + std::ops::Add<Output = T>,
// {
//     numbers.iter().copied().reduce(|acc, n| acc + n).unwrap()
// }

fn sum<'a, T>(numbers: &'a [T]) -> T
where
    T: std::iter::Sum<&'a T>,
{
    numbers.iter().sum()
}
