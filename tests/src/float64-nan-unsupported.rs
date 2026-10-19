//@ [!lean] skip
//@ [lean] known-failure
// Charon's textual encoding does not retain NaN payload bits (#1507).
pub fn nan() -> f64 {
    const { 0.0f64 / 0.0f64 }
}
