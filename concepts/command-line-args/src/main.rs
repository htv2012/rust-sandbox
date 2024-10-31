use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("\nargs:");
    let mut counter = 0;
    for arg in env::args() {
        println!("args[{}]={}", counter, arg);
        counter += 1;
    }

    println!("args:");
    let mut counter = 0;
    for arg in args {
        println!("args[{}]={}", counter, arg);
        counter += 1;
    }
}
