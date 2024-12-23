use std::process::Command;

/// Execute the same command in different directories
fn main() {
    let mut command = Command::new("ls");

    // Execute in current directory
    println!("\nls output in current dir:");
    command.arg("-l").status().expect("failed at current dir");
    println!();

    // Execute in root
    println!("\nls output in root:");
    command.current_dir("/");
    command.status().expect("failed at current dir");
    println!();
}
