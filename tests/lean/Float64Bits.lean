module
import Float64

set_option autoImplicit false

/- Check the actual extracted constants against their source binary64 words. -/
example : float64.positive_zero = Aeneas.Std.Result.ok (Float.ofBits 0) := rfl

example : float64.negative_zero =
    Aeneas.Std.Result.ok (Float.ofBits 9223372036854775808) := rfl

example : float64.literal =
    Aeneas.Std.Result.ok (Float.ofBits 4591870180066957722) := rfl

example : float64.subnormal = Aeneas.Std.Result.ok (Float.ofBits 1) := rfl

example : float64.max_finite =
    Aeneas.Std.Result.ok (Float.ofBits 9218868437227405311) := rfl

example : float64.positive_infinity =
    Aeneas.Std.Result.ok (Float.ofBits 9218868437227405312) := rfl

example : float64.negative_infinity =
    Aeneas.Std.Result.ok (Float.ofBits 18442240474082181120) := rfl
