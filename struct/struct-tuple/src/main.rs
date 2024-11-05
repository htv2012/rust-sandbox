//! Struct with tuple

#[derive(Debug)]
struct Point(i32, i32);

fn main() {
    let point = Point(-5, 17);
    println!("\n# Define a new tuple struct");
    println!("{:?}", point);

    println!("\n# Access as field");
    println!("x={}, y={}", point.0, point.1);
}
