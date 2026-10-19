//@ [!lean] skip
#![feature(register_tool)]
#![register_tool(verify)]

// The numerical part of #828's reproducer. Debug formatting is outside this test.
#[derive(Clone, PartialEq)]
pub enum BucketReduction {
    Failure,
    Success,
    OverConsumption(f64),
}

pub fn test_create() -> BucketReduction {
    BucketReduction::OverConsumption(1.5)
}

pub fn test_divide(x: f64, y: f64) -> BucketReduction {
    BucketReduction::OverConsumption(x / y)
}

// The issue's follow-up must use a concrete Float type, even with autoImplicit off.
pub fn pick(x: f64, y: f64, first: bool) -> f64 {
    if first {
        x
    } else {
        y
    }
}

pub fn clone_values(values: &Vec<f64>) -> Vec<f64> {
    values.clone()
}

#[verify::test]
pub fn test_bucket() {
    assert!(test_create() == BucketReduction::OverConsumption(1.5));
    assert!(test_divide(3.0, 2.0) == test_create());
    assert!(test_create().clone() == test_create());
    assert!(pick(1.0, 2.0, true) == 1.0);
    assert!(pick(1.0, 2.0, false) == 2.0);
    assert!(BucketReduction::OverConsumption(0.0) == BucketReduction::OverConsumption(-0.0));
    let nan_bucket = test_divide(0.0, 0.0);
    assert!(nan_bucket != nan_bucket);
    let mut source = Vec::new();
    source.push(1.0);
    source.push(-0.0);
    let values = clone_values(&source);
    assert!(values[0] == 1.0);
    assert!(1.0 / values[1] == -1.0 / 0.0);
}

#[cfg(test)]
mod tests {
    #[test]
    fn native_bucket() {
        super::test_bucket();
    }
}
