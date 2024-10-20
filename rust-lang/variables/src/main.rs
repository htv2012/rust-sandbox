fn main() {
    // Types can be inferred
    let name = "Hai";

    // This variable is immutable (act like a const in other languages)
    // That means the following will fail:
    //     name = "John";

    // This variable, with `mut` is mutable
    let mut weight = 150.0;

    println!("\n# Name\n{}", name);
    println!("\n# Weight in pounds\n{}", weight);
    weight = weight / 2.2;
    println!("\n# Weight in kilograms\n{}", weight);
}
