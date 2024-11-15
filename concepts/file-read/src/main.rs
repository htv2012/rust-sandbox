//! Demo
//! - Use std::fs::read_to_string to simplify reading from file
//! - Error handling

use std::fs::read_to_string;

fn main() {
    let file_names = vec!["src/main.rs", "/tmp/noread.txt"];
    for file_name in file_names {
        println!("\n# cat {}", file_name);
        match read_to_string(file_name) {
            Ok(content) => println!("{}", content),
            Err(error) => eprintln!("{}: {}", file_name, error),
        }
    }
}

