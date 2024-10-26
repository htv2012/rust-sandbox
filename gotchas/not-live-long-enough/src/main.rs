fn main() {
    let x;
    {
        let y = 24;
        x = &y;
    }
    println!("x={}", x);
}
