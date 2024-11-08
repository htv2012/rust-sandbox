#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
}

impl Shape {
    fn circumference(&self) -> f64 {
        match self {
            Shape::Circle(radius) => radius * 2.0 * std::f64::consts::PI,
            Shape::Square(side) => side * 4.0,
        }
    }
}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(1.0)];
    println!("\n# Vector");
    println!("{:?}", shapes);

    println!("\n# Total Areas");
    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(side) => side * side,
        })
        .sum();
    println!("{:.2}", total_area);

    println!("\n# Total Circumferences");
    let total_circumferences: f64 = shapes.iter().map(|shape| shape.circumference()).sum();
    println!("{:.2}", total_circumferences);
}
