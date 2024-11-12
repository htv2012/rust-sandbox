use home::{cargo_home, rustup_home, home_dir};

fn main() {
    println!("\n# cargo_home()");
    match cargo_home() {
        Ok(path) => println!("{:?}", path),
        Err(error) => eprintln!("{}", error),
    };

    println!("\n# home_dir()");
    match home_dir() {
        Some(path) => println!("{:?}", path),
        None => eprintln!("Failed to get home dir"),
    }

    println!("\n# rustup_home()");
    match rustup_home() {
        Ok(path) => println!("{:?}", path),
        Err(error) => eprintln!("{}", error),
    };

}
