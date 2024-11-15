//! Demo: Use a struct as an error type

#[derive(Debug)]
struct DivideByZeroError;

fn main() {
    let denominators = vec![2.0, 3.0, 4.0, 0.0, 5.0];

    for denominator in denominators {
        print!("1.0/{} = ", denominator);
        match div(1.0, denominator) {
            Ok(value) => println!("{}", value),
            Err(error) => println!("{:?}", error),
        }
    }
}

fn div(a: f64, b: f64) -> Result<f64, DivideByZeroError> {
    if b == 0.0 {
        Err(DivideByZeroError)
    } else {
        Ok(a / b)
    }
}
