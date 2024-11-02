fn main() {
    let sentence = "the quick brown fox  jumps over the lazy dog".to_string();
    println!("\n# Whole string");
    println!("{}", sentence);

    println!("\n# fist 19 chars");
    println!("'{}'", &sentence[0..19]);

    println!("\n# Concatenate using format");
    let story = format!("Short story: {}", sentence);
    println!("{}", story);

    println!("\n# Split words, note where there are 2 spaces");
    let words = sentence.split(' ').collect::<Vec<&str>>();
    println!("{:?}", words);

    println!("\n# Reverse string");
    let reversed: String = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);
}
