/// PartialEq needed for == comparison
#[derive(PartialEq)]
enum DiskType {
    SSD,
    HDD,
}

#[derive(Debug)]
enum DiskSize {
    KB(u32),
    MB(u32),
    GB(u32),
}

fn main() {
    println!("\n# Define disk type");
    let disk_type = DiskType::SSD;

    // The following needed for partial equal
    if disk_type == DiskType::SSD {
        println!("SSD");
    } else {
        println!("HDD");
    }

    // Without partial equal, we will have to do this:
    match disk_type {
        DiskType::SSD => println!("SSD"),
        DiskType::HDD => println!("HDD"),
    }

    println!("\n# DiskSize Enum");
    let sizes = [DiskSize::KB(128), DiskSize::MB(4), DiskSize::GB(32)];
    for size in sizes {
        print_disk_size(&size);
    }
}

fn print_disk_size(size: &DiskSize) {
    match size {
        DiskSize::KB(value) => println!("{}KB", value),
        DiskSize::MB(value) => println!("{}MB", value),
        DiskSize::GB(value) => println!("{}GB", value),
    }
}
