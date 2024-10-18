use std::io::Write;

fn main() {
    let mut answer = String::new();
    print!("Do you like Rust? ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut answer).expect("Read failed");

    match answer.trim() {
        "yes" => println!("Me too!"),
        "no" => println!("May I persuade you?"),
        _ => println!("Ah, the art of answering by not answering"),
    }
}
