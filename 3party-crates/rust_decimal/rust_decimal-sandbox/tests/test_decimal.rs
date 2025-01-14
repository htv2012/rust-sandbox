use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

#[test]
fn test_dec_macro() {
    let number = dec!(-1.23) + dec!(3.45);
    assert_eq!(number, dec!(2.22));
    assert_eq!(number.to_string(), "2.22");
}

#[test]
fn test_from_string() {
    let n = Decimal::from_str("2.02").unwrap();
    assert_eq!("2.02", n.to_string());
}

#[test]
fn test_from_ints() {
    let n = Decimal::new(202, 2);
    assert_eq!(dec!(2.02), n);
}

#[test]
fn test_simple_math() {
    let amount = Decimal::new(100, 0);
    let tip_percent = Decimal::new(18, 2);
    let amount_with_tip = amount + (amount * tip_percent);
    assert_eq!(dec!(118.0), amount_with_tip);
}

#[test]
fn test_to_f64() {
    let amount = Decimal::new(195, 2);
    let expected: f64 = 1.95;
    assert_eq!(amount.to_f64(), Some(expected));
}
