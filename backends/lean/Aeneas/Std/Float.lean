module
public import Aeneas.Std.Core.Core
public import Aeneas.Std.Core.Cmp
public section

/-!
# Native binary64 model

Rust `f64` is translated to Lean `Float`. Each primitive performs one native
binary64 operation; exceptional values are preserved. Callers implement their
own rejection policy. NaN payload selection remains a native primitive boundary.
-/
namespace Aeneas.Std

@[expose, rust_fun "core::f64::{f64}::is_finite" -canFail]
def core.f64.is_finite (x : Float) : Bool := x.isFinite

@[expose, rust_fun "core::f64::{f64}::abs" -canFail]
def core.f64.abs (x : Float) : Float := x.abs

@[expose, rust_fun "std::f64::{f64}::sqrt" -canFail]
def std.f64.sqrt (x : Float) : Float := x.sqrt

@[expose, rust_fun "core::clone::impls::{core::clone::Clone<f64>}::clone" -canFail]
def core.clone.impls.CloneF64.clone (x : Float) : Float := x

@[expose, reducible, rust_trait_impl "core::clone::Clone<f64>"]
def core.clone.CloneF64 : core.clone.Clone Float where
  clone := fun x => .ok x
  clone_from := fun _ x => .ok x

@[expose, reducible, rust_trait_impl "core::marker::Copy<f64>"]
def core.marker.CopyF64 : core.marker.Copy Float where
  cloneInst := core.clone.CloneF64

@[expose, rust_fun "core::cmp::impls::{core::cmp::PartialEq<f64, f64>}::eq" -canFail]
def core.cmp.impls.PartialEqF64.eq (x y : Float) : Bool := x == y

@[expose, rust_fun "core::cmp::impls::{core::cmp::PartialEq<f64, f64>}::ne" -canFail]
def core.cmp.impls.PartialEqF64.ne (x y : Float) : Bool := x != y

@[expose, reducible, rust_trait_impl "core::cmp::PartialEq<f64, f64>"]
def core.cmp.PartialEqF64 : core.cmp.PartialEq Float Float where
  eq := liftFun2 core.cmp.impls.PartialEqF64.eq
  ne := liftFun2 core.cmp.impls.PartialEqF64.ne

end Aeneas.Std
