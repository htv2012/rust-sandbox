// Demo: Shadowing: Reassign variables type and values
fn main() {
    let x = 43;
    println!("# Original declaration, as int, x={}", x);

    let x = "forty three";
    println!("# Shadowing: re-define type and value to string, x={}", x);

    {
        let x = 43.3;
        println!("# Inner scope, re-define as float, x={}", x);
    }

    println!("# Exit inner scope, x={}", x);
}
