use cli_utils::colors::{ColorString, Color};

#[test]
fn test_red() {
    let mut cs = ColorString{
        color: Color::Red,
        string: String::from("bull"),
        colorized: String::new(),
    };
    assert_eq!(cs.colorized, "");
    cs.paint();
    assert_eq!(cs.colorized, "\x1b[31mbull\x1b[0m");
}