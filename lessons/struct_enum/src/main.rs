#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug)]
enum OrderStatus {
    Paid { amount: u32 },
    Sent,
    Delivered,
    Dispured(String),
}

impl OrderStatus {
    fn info(&self) {
        match self {
            Self::Paid { amount } => println!("Paid, {amount}"),
            Self::Sent => println!("Sent"),
            Self::Delivered => println!("Delivered"),
            Self::Dispured(reason) => println!("Disputed with reason: {reason}"),
        }
    }
}

#[derive(Debug)]
struct Order {
    customer: String,
    status: OrderStatus,
}

fn main() {
    let status = OrderStatus::Sent;
    demo(&status);

    let order = Order {
        customer: String::from("John Doe"),
        status,
    };
    println!("{order:#?}");

    let status2 = OrderStatus::Dispured(String::from("broken"));
    println!("{status2:#?}");

    let status3 = OrderStatus::Paid { amount: 10000 };
    println!("{status3:#?}");
    status3.info();

    let value: Option<i8> = Some(32);
    // let value2: Option<i8> = None;

    // let result = value.unwrap_or(0) + 1;

    // match value {
    //     Some(a) => {
    // let result = a + 5;
    // println!("{result}");
    //     }
    //     None => (),
    // }

    if let Some(a) = value {
        let result = a + 5;
        println!("{result}");
    }
}

fn demo(status: &OrderStatus) {
    println!("{status:#?}");
}
