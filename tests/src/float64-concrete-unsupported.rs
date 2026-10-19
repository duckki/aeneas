//@ [!lean] skip
//@ [lean] known-failure
//@ [lean] aeneas-args=-test-units
// Concrete evaluation must not compare textual float records structurally.
pub fn equal(x: f64, y: f64) -> bool {
    x == y
}

pub fn test_zero_equality() {
    assert!(equal(0.0, -0.0));
}
