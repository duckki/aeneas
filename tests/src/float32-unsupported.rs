//@ [!lean] skip
//@ [lean] known-failure
// Reject the type itself instead of emitting an undefined implicit F32 parameter.
pub fn pick(x: f32, y: f32, first: bool) -> f32 {
    if first {
        x
    } else {
        y
    }
}
