fn main() {
    println!("\n# Original");
    let mut vector = vec![1, 2, 3];
    println!("vector={:?}", vector);

    println!("\n# Push 4");
    vector.push(4);
    println!("vector={:?}", vector);

    let vector2 = vec![5, 6];
    println!("\n# Extend with vector2={:?}", vector2);
    vector.extend(&vector2);
    println!("vector={:?}", vector);
    println!("vector2={:?}", vector2);

    let mut vector3 = vec![7, 8, 9];
    println!("\n# Drain vector3={:?}", vector3);
    vector.extend(vector3.drain(..));
    println!("vector={:?}", vector);
    println!("vector3={:?}", vector3);

    println!("\n# Insert at index 0, value 99");
    vector.insert(0, 99);
    println!("vector={:?}", vector);
}
