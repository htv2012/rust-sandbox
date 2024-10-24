use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_name = "data.txt";
    let file = File::open(file_name);
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("{}: {}", file_name, error)
            }
            _ => {
                panic!("{}: {}", file_name, error)
            }
        },
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}
