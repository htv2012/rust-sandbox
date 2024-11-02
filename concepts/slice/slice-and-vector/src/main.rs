//! Demo: Slice in relation to vector
use std::panic;

fn main() {
    println!("\n# Whole vector");
    let mut vector = vec![1, 2, 3, 4, 5];
    println!("{:?}", vector);

    println!("\n# Slice 1..3");
    let slice = &mut vector[1..3];
    println!("{:?}", slice);

    println!("\n# Modify slice[0]");
    slice[0] = 20;
    println!("{:?}", slice);

    println!("\n# Vector after slice modification");
    println!("{:?}", vector);

    println!("\n# Other slice, 2..4, which is overlapped with the first slice");
    let other_slice = &vector[2..4];
    println!("{:?}", other_slice);

    // This will fail
    // other_slice[0] = 30;
}
