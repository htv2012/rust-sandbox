// TODO: modify only this function.
fn copy_and_return<'a>(vector: &mut Vec<String>, value: &'a str) -> &'a str{
    vector.push(String::from(value));
    return &value;
}

fn main() {
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    );

    // Prints to ensure that all values are still valid
    println!("name1={name1}");
    println!("name2={name2}");
    println!("name3={name3}");
    println!("names={:?}", names);
}
