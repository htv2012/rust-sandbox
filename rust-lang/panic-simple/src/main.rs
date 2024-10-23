fn find_youngest_age(ages: Vec<i32>) {
    let youngest = ages[0];
    for age in ages {
        println!("Processing: {age}");
        if age < 0 {
            panic!("Age cannot be less than zero!");
        }
        if age < youngest {
            youngest = age
        }
    }
    youngest
}

fn main() {
    let youngest = find_youngest_age(vec![2, 25, 3, -5, 9]);
    println!("Youngest: {}", youngest);
}

