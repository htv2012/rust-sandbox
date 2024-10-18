use std::io::Write;
fn main() {
    wrong_way();
    right_way1();
    right_way2();
}

fn wrong_way() {
    // This is the wrong way to do it
    // read_line() keep appending to the string
    // Secondly, the string will end with a newline character
    // which we need to call trim() to remove it

    println!("\n# Wrong Way to read_line()");

    let mut name = String::new();
    for _ in 0..3 {
        print!("Enter a name: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut name).unwrap();
        println!("Hello, '{}'", name);
    }
}

fn right_way1() {
    // Right way #1: Use a fresh string every time we call read_line()

    println!("\n# Right Way #1: Use a fresh string every time we call read_line()");
    for _ in 0..3 {
        let mut name = String::new();
        print!("Enter a name: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut name).unwrap();
        println!("Hello, '{}'", name.trim());
    }
}

fn right_way2() {
    // Right way #2: Clear the string before we call read_line()

    println!("\n# Right way #2: Clear the string before we call read_line()");

    let mut name = String::new();
    for _ in 0..3 {
        print!("Enter a name: ");
        std::io::stdout().flush().unwrap();
        name.clear();
        std::io::stdin().read_line(&mut name).unwrap();
        println!("Hello, '{}'", name.trim())
    }

}