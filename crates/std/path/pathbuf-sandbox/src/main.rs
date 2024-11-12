use std::path::PathBuf;

fn main() {
    from_str();
}

fn from_str() {
    let path = PathBuf::from("./src/main.rs");
    println!("\n# path={}", path.display());

    match path.canonicalize() {
        Ok(path) => println!("canonicalize() -> {}", path.display()),
        Err(error) => println!("canonicalize() -> error: {}", error),
    };

    println!("exists() -> {}", path.exists());
    println!("is_dir() -> {}", path.is_dir());
    println!("is_file() -> {}", path.is_file());
}
