use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("./src/main.rs");
    show_attributes(&path);

    let mut path = PathBuf::new();
    path.push("/");
    path.push("tmp");
    show_attributes(&path);

    path.push("file");
    let path2 = path.with_extension("txt");
    show_attributes(&path2);
}

fn show_attributes(path: &PathBuf) {
    println!("\n# path={}", path.display());

    match path.canonicalize() {
        Ok(path) => println!("canonicalize() -> {}", path.display()),
        Err(error) => println!("canonicalize() -> error: {}", error),
    };

    println!("ends_with(main.rs) -> {}", path.ends_with("main.rs"));
    println!("ends_with(rs) -> {}", path.ends_with("rs"));
    println!("exists() -> {}", path.exists());
    println!("extension() -> {:?}", path.extension());
    println!("file_name() -> {:?}", path.file_name().unwrap());
    println!("file_stem() -> {:?}", path.file_stem().unwrap());
    println!("has_root() -> {}", path.has_root());
    println!("is_dir() -> {}", path.is_dir());
    println!("is_file() -> {}", path.is_file());
    println!("is_relative() -> {}", path.is_relative());
    println!("is_symlink() -> {}", path.is_symlink());
}
