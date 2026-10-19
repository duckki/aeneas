//@ [!lean] skip
#![feature(register_tool)]
#![register_tool(verify)]
//! Native IEEE binary64 operations: no Rust overflow panic on float arithmetic.
pub fn add(x: f64, y: f64) -> f64 {
    x + y
}
pub fn sub(x: f64, y: f64) -> f64 {
    x - y
}
pub fn mul(x: f64, y: f64) -> f64 {
    x * y
}
pub fn div(x: f64, y: f64) -> f64 {
    x / y
}
pub fn neg(x: f64) -> f64 {
    -x
}
pub fn eq(x: f64, y: f64) -> bool {
    x == y
}
pub fn ne(x: f64, y: f64) -> bool {
    x != y
}
pub fn lt(x: f64, y: f64) -> bool {
    x < y
}
pub fn le(x: f64, y: f64) -> bool {
    x <= y
}
pub fn gt(x: f64, y: f64) -> bool {
    x > y
}
pub fn ge(x: f64, y: f64) -> bool {
    x >= y
}
pub fn positive_zero() -> f64 {
    0.0
}
pub fn negative_zero() -> f64 {
    -0.0
}
pub fn literal() -> f64 {
    0.1
}
pub fn is_finite(x: f64) -> bool {
    x.is_finite()
}
pub fn abs(x: f64) -> f64 {
    x.abs()
}
pub fn sqrt(x: f64) -> f64 {
    x.sqrt()
}
pub fn subnormal() -> f64 {
    5e-324
}
pub fn max_finite() -> f64 {
    1.7976931348623157e308
}
pub fn positive_infinity() -> f64 {
    const { 1.0f64 / 0.0f64 }
}
pub fn negative_infinity() -> f64 {
    const { -1.0f64 / 0.0f64 }
}

#[verify::test]
pub fn test_arithmetic() {
    assert!(add(1.0, 2.0) == 3.0);
    assert!(sub(5.0, 2.0) == 3.0);
    assert!(mul(1.5, 2.0) == 3.0);
    assert!(div(6.0, 2.0) == 3.0);
    assert!(neg(3.0) == -3.0);
    assert!(lt(1.0, 2.0));
    assert!(le(1.0, 1.0));
    assert!(gt(2.0, 1.0));
    assert!(ge(1.0, 1.0));
}

#[verify::test]
pub fn test_ieee() {
    let nan = div(positive_zero(), positive_zero());
    assert!(ne(nan, nan));
    assert!(!eq(nan, nan));
    assert!(eq(positive_zero(), negative_zero()));
    assert!(!lt(nan, 1.0));
    assert!(!le(nan, 1.0));
    assert!(!gt(nan, 1.0));
    assert!(!ge(nan, 1.0));
    assert!(!is_finite(div(1.0, positive_zero())));
    assert!(!is_finite(nan));
    assert!(is_finite(max_finite()));
    assert!(subnormal() > 0.0);
    assert!(sqrt(4.0) == 2.0);
    assert!(abs(-2.0) == 2.0);
    assert!(!is_finite(mul(max_finite(), 2.0)));
    assert!(eq(
        div(1.0, negative_zero()),
        neg(div(1.0, positive_zero()))
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_operations() {
        test_arithmetic();
        test_ieee();
    }

    #[test]
    fn exact_literal_bits() {
        assert_eq!(positive_zero().to_bits(), 0);
        assert_eq!(negative_zero().to_bits(), 0x8000_0000_0000_0000);
        assert_eq!(literal().to_bits(), 0x3fb9_9999_9999_999a);
        assert_eq!(subnormal().to_bits(), 1);
        assert_eq!(max_finite().to_bits(), 0x7fef_ffff_ffff_ffff);
        assert_eq!(positive_infinity().to_bits(), 0x7ff0_0000_0000_0000);
        assert_eq!(negative_infinity().to_bits(), 0xfff0_0000_0000_0000);
    }
}
