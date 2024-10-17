fn main() {
    let proceed = false;
    if proceed {
        println!("Proceeding");
    } else {
        println!("Not proceeding");
    }

    let height = 161;
    if height > 180 {
        println!("{} is considered tall", height);
    } else if height > 160 {
        println!("{} is considered average", height);
    } else {
        println!("{} is considered short", height);
    }
}
