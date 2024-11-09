fn main() {
    let vector: Vec<i8> = vec![1, 2, 3];
    get_index(&vector);

    let vector: Vec<i8> = vec![];
    get_index(&vector);

    let vector: Vec<i8> = vec![9];
    get_index(&vector);
}

fn get_index(vector: &Vec<i8>) {
    println!("\n# vector={:?}", vector);
    match vector.get(2) {
        Some(value) => println!("vector.get(2) -> {}", value),
        None => println!("vector.get(2) -> None"),
    };

    match vector.first() {
        Some(value) => println!("vector.first() -> {}", value),
        None => println!("vector.first() -> None"),
    };

    match vector.last() {
        Some(value) => println!("vector.last() -> {}", value),
        None => println!("vector.last() -> None"),
    };
}
