#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

fn main() {
    let miles_list: Vec<u32> = vec![15, 999, 1000, 1001, 25000];
    for miles in miles_list {
        println!("\n# miles={}", miles);
        println!("{:?}", access_quality(miles));
    }
}

fn access_quality(miles: u32) -> Age {
    if miles < 1001 { Age::New } else { Age::Used }
}
