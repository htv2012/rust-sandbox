//! Demo: Result type

fn main() {
    demo(81.3, 3.1);
    demo(25.5, 0.0);
}

fn demo(a: f64, b: f64) {
    println!("\n# Division Demo");
    match divi(a, b) {
        Ok(result) => println!("{} / {} = {}", a, b, result),
        Err(error) => println!("{} / {} => {}", a, b, error),
    }
}

fn divi(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}
