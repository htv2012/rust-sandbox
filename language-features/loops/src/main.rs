fn main() {
    println!("\n# The while loop");
    let mut counter = 0;
    while counter < 3 {
        print!("{} ", counter);
        counter += 1;
    }
    println!();

    println!("\n# The loop loop");
    counter = 0;
    loop {
        print!("{} ", counter);
        counter += 1;
        if counter == 3 {
            break;
        }
    }
    println!();

    println!("\n# The for loop");
    for counter in 0..3 {
        print!("{} ", counter);
    }
    println!();

    println!("\n# The reverse for loop");
    for counter in (0..3).rev() {
        print!("{} ", counter);
    }
    println!();
}
