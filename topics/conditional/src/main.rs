fn main() {
    let proceed = false;
    if proceed {
        println!("Proceeding");
    } else {
        println!("Not proceeding");
    }

    let height = 136;
    let gender = "female";
    let judgement = if gender == "male" {
        if height > 180 {
            "tall"
        } else if height > 160 {
            "average"
        } else {
            "short"
        }
    } else if gender == "female" {
        if height > 155 {
            "tall"
        } else if height > 135 {
            "average"
        } else {
            "short"
        }
    } else {
        "unknown"
    };

    println!("{}cm is consider {} for a {}", height, judgement, gender);
}
