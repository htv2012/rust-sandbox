//! Demmo: ownership

fn main() {
    // name is owned by function main
    let name = String::from("Dolly");

    // We tell say_hello that we will let it borrow name for a moment
    say_hello(&name);
    // After the function returned, we again own the variable name

    // By passing `name` to `say_bye()`, `main()` no longer own `name`
    // In other word, ownership has been moved from `main()` to `say_bye()`
    say_bye(name);

    // That means this will fail:
    // println!("Hello, {name}");
}

fn say_hello(the_name: &String) {
    // This function only 'borrow' the_name. It does not take ownership
    // of the variable
    println!("Hello {the_name}");
}

fn say_bye(the_name: String) {
    // `say_bye()` takes ownership of `the_name`
    println!("Bye {the_name}");

    // When `say_bye()` goes out of scope, so is `the_name`,
    // and Rust will call a special function `drop()` to reclaim memory
}
