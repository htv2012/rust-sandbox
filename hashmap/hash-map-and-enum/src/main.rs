use std::collections::HashMap;

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    Automatic,
    CVT,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    transmission: Transmission,
    roof: bool,
    mileage: u32,
    age: Age,
}

impl Car {
    fn print(self: &Car) {
        println!("Color: {}", self.color);
        println!("Transmission: {:?}", self.transmission);
        println!("Roof: {}", self.roof);
        println!("Mileage: {}", self.mileage);
        println!("Age: {:?}", self.age);
    }
}

/// Use a hashmap to keep track of orders
/// orders = order number : car
fn main() {
    let mut orders: HashMap<u32, Car> = HashMap::new();
    orders.insert(
        7375,
        Car {
            color: String::from("red"),
            transmission: Transmission::Automatic,
            roof: true,
            mileage: 19_500,
            age: Age::Used,
        },
    );
    orders.insert(
        7376,
        Car {
            color: String::from("blue"),
            transmission: Transmission::Manual,
            roof: false,
            mileage: 65_320,
            age: Age::Used,
        },
        );


    for (order_number, car) in orders {
        println!("\n# Order #{}", order_number);
        car.print();
    }
}
