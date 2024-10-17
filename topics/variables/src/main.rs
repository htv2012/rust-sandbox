fn main() {
    // Types can be inferred
    let name = "Hai";

    // This variable is immutable (act like a const in other languages)
    // That means the following will fail:
    //     name = "John";
    
    let weight_pound = 150.0;
    let weight_kg = weight_pound / 2.2;

    println!("\n# Name\n{}", name);
    println!("\n# Weight in pounds\n{}", weight_pound);
    println!("\n# Weight in kilograms\n{}", weight_kg);
}
