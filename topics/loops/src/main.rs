use std::io::{self, Write};

fn main() {
    // The `while` loop
    let mut x = 0;
    while x < 5 {
        println!("x={}", x);
        x += 1;
    }
    print!("=====================\n");

    // The `loop` loop
    let mut name = String::new();
    loop {
        name.clear();
        print!("Enter your name (q to quit): ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut name)
            .expect("Failed to read name");
        
        if name.trim() == "q" {
            break;
        }

        print!("Hello, {}", name);
    }
}
