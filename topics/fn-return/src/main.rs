//! In this program, we demonstrate 2 kinds of functions:
//!
//! 1. Those that returns a value
//! 2. Those that don't (a unit function, e.g. `report`)
//!
//! Other notes
//! - To return a value at the end of the function, just name the return value
//! - Do not add the final semicolon

use std::io::{self, BufRead};

fn main() {
    // let values = [1, 2, 3, 4, 5];
    let values = read_numbers();
    let total = sum(&values);
    report_sum(&values, total);
    let average_value = average(&values);
    report_average(&values, average_value);
}

/// Get the user input and return a vector of numbers.
fn read_numbers() -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    println!("Enter a list of integers, one per line");
    for line in io::stdin().lock().lines() {
        match line {
            Ok(text) => match text.trim().parse::<i32>() {
                Ok(number) => numbers.push(number),
                Err(_) => {
                    eprint!("Not a valid number: {}", text);
                    println!();
                }
            },
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        }
    }
    numbers
}

/// Calculate the sum of a vector.
fn sum(numbers: &Vec<i32>) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

/// Return an average value of a vector.
fn average(numbers: &Vec<i32>) -> f64 {
    if numbers.len() == 0 {
        return 0.0;
    }

    let mut total = 0.0;
    for &number in numbers {
        total += number as f64;
    }

    total / numbers.len() as f64
}

/// Report the sum
fn report_sum(numbers: &Vec<i32>, the_sum: i32) {
    println!("Sum of {:?} is {}", numbers, the_sum);
}

/// Report the average
fn report_average(numbers: &Vec<i32>, average_value: f64) {
    println!("Average of {:?} is {}", numbers, average_value);
}
