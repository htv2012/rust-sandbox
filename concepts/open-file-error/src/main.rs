use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_name = "data.txt";
    match File::open(file_name) {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                eprintln!("{}: {}", file_name, error)
            }
            _ => {
                eprintln!("{}: {}", file_name, error)
            }
        },
    };
}
