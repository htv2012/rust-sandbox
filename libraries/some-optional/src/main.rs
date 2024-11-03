fn main() {
    let a: i32 = 10;
    let mut b: i32 = 0;

    // Consume optional using match
    println!("\n# {} / {}", a, b);
    match divide(a, b) {
        Some(number) => println!("{}", number),
        None => println!("Divide by zero"),
    }

    // Consume optional using if
    b = 3;
    println!("\n# {} / {}", a, b);
    if let Some(number) = divide(a, b) {
        println!("{}", number);
    } else {
        println!("Devide by zero");
    }
}

fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}
