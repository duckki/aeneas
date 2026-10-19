module
import Issue828Float

set_option autoImplicit false

open Aeneas.Std

/- `@` makes an accidentally auto-bound implicit float type visible here. -/
example : Float → Float → Bool → Result Float := @issue_828_float.pick

example : issue_828_float.test_create =
    Result.ok (issue_828_float.BucketReduction.OverConsumption (1.5 : Float)) := rfl

example (x y : Float) : issue_828_float.test_divide x y =
    Result.ok (issue_828_float.BucketReduction.OverConsumption (Float.div x y)) := rfl
