//! In this program, we demonstrate 2 kinds of functions:
//!
//! 1. Those that returns a value
//! 2. Those that don't (a unit function)
//!
//! Other notes
//! - To return a value at the end of the function, just name the return value
//! - Do not add the final semicolon

fn main() {
    let values = [1, 2, 3, 4, 5];
    let total = sum(&values);
    report(&values, total);
}

/// Demonstrate function with return value.
///
/// Simply name the return value as the last line of the function will return
/// its value. Do not add the final semicolon
fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

/// Demonstrate a unit function: Function that do not return a value
fn report(numbers: &[i32], the_sum: i32) {
    println!("Sum of {:?} is {}", numbers, the_sum);
}
