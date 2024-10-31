// Demo: Shadowing: Reassign variables type and values
fn main() {
    // Original
    let x = 43;
    println!("\n# Original declaration, as int");
    println!("x={}", x);

    let x = "forty three";
    println!("\n# Shadowing: redefine type and value to string");
    println!("x={}", x);

    {
        println!("\n# Inner scope, dedefine as float");
        let x = 43.3;
        println!("x={}", x);
    }

    println!("\n# Exit inner scope");
    println!("x={}", x);
}
