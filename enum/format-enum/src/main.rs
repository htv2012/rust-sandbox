//! Demo how to format an enum with {:width.precision}

use std::fmt;

#[derive(Debug)]
enum FileSize {
    B(f64),
    KB(f64),
    MB(f64),
    GB(f64),
}

impl fmt::Display for FileSize {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = match formatter.width() {
            Some(value) => value,
            None => 0,
        };
        let precision = match formatter.precision() {
            Some(value) => value,
            None => 0,
        };

        let size: &f64;
        let suffix: String;
        match self {
            FileSize::B(value) => {
                size = value;
                suffix = String::from("B");
            }
            FileSize::KB(value) => {
                size = value;
                suffix = String::from("KB");
            }
            FileSize::MB(value) => {
                size = value;
                suffix = String::from("MB");
            }
            FileSize::GB(value) => {
                size = value;
                suffix = String::from("GB");
            }
        };
        write!(
            formatter,
            "{:w$.p$}{}",
            size,
            suffix,
            w = width,
            p = precision
        )
    }
}

impl FileSize {
    fn from_bytes(bytes: f64) -> Self {
        match bytes {
            0.0..1024.0 => FileSize::B(bytes),
            1024.0..1_048_576.0 => FileSize::KB(bytes / 1024.0),
            1_048_576.0..1_073_741_824.0 => FileSize::MB(bytes / 1024.0 / 1024.0),
            _ => FileSize::GB(bytes / 1024.0 / 1024.0 / 1024.0),
        }
    }
}

fn main() {
    let sizes_in_bytes: Vec<f64> = vec![512.0, 2000.0, 2_000_000.0, 1_999_999_999.0];
    for size in sizes_in_bytes {
        let file_size = FileSize::from_bytes(size);
        println!("\n# Size={:?}", file_size);
        println!("{:12.1}", file_size);
    }
}
