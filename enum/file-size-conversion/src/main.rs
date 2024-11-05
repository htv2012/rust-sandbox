//! https://github.com/alfredodeza/rust-structs-types-enums/blob/main/lab.md

use std::collections::HashMap;
use std::env;
use std::process;

#[derive(Debug)]
enum FileSize {
    B(f64),
    KB(f64),
    MB(f64),
    GB(f64),
}

impl FileSize {
    fn new(amount: f64, unit: String) -> FileSize {
        return match unit.as_str() {
            "B" => FileSize::B(amount),
            "KB" => FileSize::KB(amount),
            "MB" => FileSize::MB(amount),
            "GB" => FileSize::GB(amount),
            &_ => todo!(),
        };
    }

    fn to_sizes(self: &FileSize) -> HashMap<String, f64> {
        let mut sizes: HashMap<String, f64> = HashMap::new();
        let size_bytes = match self {
            FileSize::B(value) => &value,
            FileSize::KB(value) => &(value * 1024.0),
            FileSize::MB(value) => &(value * 1024.0 * 1024.0),
            FileSize::GB(value) => &(value * 1024.0 * 1024.0 * 1024.0),
        };

        sizes.insert(String::from("bytes"), *size_bytes);
        sizes.insert(String::from("kilobytes"), size_bytes / 1024.0);
        sizes.insert(String::from("megabytes"), size_bytes / 1024.0 / 1024.0);
        sizes.insert(
            String::from("gigabytes"),
            size_bytes / 1024.0 / 1024.0 / 1024.0,
        );
        sizes
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("Please supply an argument, for example: '24.3 MB'");
        process::exit(1);
    }
    let amount_unit: Vec<&str> = args[1].split(" ").collect();

    match amount_unit[0].parse::<f64>() {
        Ok(amount) => {
            let unit = amount_unit[1].to_uppercase();
            println!("\n# Convert from {}{}", amount, unit);
            let file_size = FileSize::new(amount, unit);

            // Convert to other sizes
            let sizes = file_size.to_sizes();
            for (unit2, amount2) in sizes {
                println!("{:14.2} {}", amount2, unit2);
            }
        }
        Err(error) => eprintln!("Canot convert {} to float: {}", amount_unit[0], error),
    }
}
