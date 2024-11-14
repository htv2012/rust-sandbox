fn main() {
    let numbers = vec![1, 3, 9];
    println!("\n# Declare a vector");
    println!("{:?}", numbers);

    println!("\n# Print using a loop");
    for number in numbers {
        print!("{} ", number);
    }
    println!();

    let mut numbers = vec![-9; 3];
    println!("\n# Declare a vector with 3 -9");
    println!("{:?}", numbers);

    println!("\n# Push -8, -7, -6");
    numbers.push(-8);
    numbers.push(-7);
    numbers.push(-6);
    println!("{:?}", numbers);

    println!("\n# Pop 7 times, -99999 means no more to pop");
    for _ in 0..7 {
        // println!("{} => {:?}", i, numbers.pop());
        let value = match numbers.pop() {
            Some(number) => number,
            None => -99999,
        };
        print!("{} ", value);
    }
    println!();

    let numbers = vec![1, 3, 9];
    println!("\n# Indexing into a vector");
    for i in 0..numbers.len() {
        print!("{} ", numbers[i]);
    }
    println!();
}
