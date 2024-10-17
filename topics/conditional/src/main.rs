fn main() {
    let proceed = false;
    if proceed {
        println!("Proceeding");
    } else {
        println!("Not proceeding");
    }

    let height = 156;
    let gender = "female";
    if gender == "male" {

        if height > 180 {
            println!("{}cm is considered tall for a male", height);
        } else if height > 160 {
            println!("{}cm is considered average for a male", height);
        } else {
            println!("{}cm is considered short for a male", height);
        }
    } else if gender=="female" {
        if height > 155 {
            println!("{}cm is considered tall for a female", height);
        } else if height > 135 {
            println!("{}cm is considered average for a female", height);
        } else {
            println!("{}cm is considered short for a female", height);
        }
    }
}
