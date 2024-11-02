//! Demo: How to define and use a struct

struct User {
    uid: u32,
    alias: String,
}

fn main() {
    let user = User {
        uid: 501,
        alias: String::from("anna"),
    };
    println!("Hello, {}, here are your info:", user.alias);
    println!("---");
    print_user(&user);
}

fn print_user(user: &User) {
    println!("UID: {}", user.uid);
    println!("Alias: {}", user.alias);
}
