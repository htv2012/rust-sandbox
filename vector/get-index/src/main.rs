fn main() {
    println!("\n# Create a vector");
    let mut vector = vec![1, 2, 3, 4, 5];
    println!("{:?}", vector);

    let index: usize = 2;
    println!("\n# Get index {}", index);
    let value = vector.get(index).unwrap();
    println!("{}", value);

    println!("\n# Get last value");
    let last = vector.last().unwrap();
    println!("{}", last);

    println!("\n# Get last value from an empty vector");
    vector.clear();
    if let Some(value) = vector.last() {
        eprintln!("Impossible! Get last value returns {}", value);
    } else {
        println!("Vector is empty");
    }
}
