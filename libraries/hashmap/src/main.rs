use std::collections::HashMap;

fn main() {
    let mut reviews: HashMap<String, String> = HashMap::new();
    reviews.insert(
        String::from("Ancient Roman History"),
        String::from("Very acurate."),
    );
    reviews.insert(
        String::from("Cooking with Rhubarb"),
        String::from("Sweet Recipes."),
    );
    reviews.insert(
        String::from("Programming in Rust"),
        String::from("Great examples."),
    );
    print_map(&reviews);
    lookup(&reviews);
    remove_item(&mut reviews);
}

fn print_map(map: &HashMap<String, String>) {
    println!("\n# The content of the HashMap");
    for (key, value) in map {
        println!("- {}: {}", key, value);
    }
}

fn lookup(map: &HashMap<String, String>) {
    println!("\n# Look up");
    let book: &str = "Programming in Rust";
    match map.get(book) {
        Some(review) => println!("- {}: {}", book, review),
        None => println!("- {}: no review found", book),
    }

    let not_found = String::from("The Joy of cooking");
    match map.get(&not_found) {
        Some(review) => println!("- {}: {}", not_found, review),
        None => println!("- {}: no review found", not_found),
    }
}

fn remove_item(map: &mut HashMap<String, String>) {
    let obsolete: &str = "Ancient Roman History";
    println!("\n# Remove title '{}'", obsolete);
    map.remove(obsolete);
    for (key, value) in map {
        println!("- {}: {}", key, value);
    }
}
