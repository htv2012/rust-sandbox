//! Demmo: ownership

fn main() {
    // name is owned by function main
    let name = String::from("world");

    // By passing `name` to `say_hello()`, `main()` no longer own `name`
    // In other word, ownership has been moved from `main()` to `say_hello()`
    say_hello(name);

    // That means this will fail:
    // println!("Hello, {name}");
}

fn say_hello(name: String) {
    // `say_hello()` takes ownership of `name`
    println!("Hello, {name}");

    // When `say_hello()` goes out of scope, so is `name`,
    // and Rust will call a special function `drop()` to reclaim memory
}