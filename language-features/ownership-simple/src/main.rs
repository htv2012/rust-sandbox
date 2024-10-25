//! A simple demonstration of ownership.

fn main() {
    let s1 = String::from("Hello");
    println!("s1={}", s1);

    // s2 now takes ownership of the memory allocation previously belong to s1
    // Thus, s1 is now invalid
    let s2 = s1;
    println!("s2={}", s2);

    // This will fail:
    // println!("s1={}", s1);
}
