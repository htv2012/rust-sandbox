fn main() {
    // Types can be inferred
    let name = "Hai";
    let weight_pound = 150.0;
    let weight_kg = weight_pound / 2.2;

    println!("\n# Name\n{}", name);
    println!("\n# Weight in pounds\n{}", weight_pound);
    println!("\n# Weight in kilograms\n{}", weight_kg);
}
