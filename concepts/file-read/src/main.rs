use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    match File::open(file_name) {
        Ok(file) => {
            let mut line_number = 1;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                println!("{:4}: {}", line_number, line.unwrap());
                line_number += 1;
            }
        }
        Err(error) => eprintln!("{}: {}", error, file_name)
    }
}
