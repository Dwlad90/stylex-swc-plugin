//! Calls `hypot` through the shim in `glibc_compat.rs`.
//!
//! The shim is a jump in assembly, so a wrong jump target crashes or gives
//! garbage. These calls find that.
//! `black_box` stops the compiler from folding a call at build time, which
//! would skip the shim.

use std::hint::black_box;

fn hypot(x: f64, y: f64) -> f64 {
  black_box(x).hypot(black_box(y))
}

#[test]
fn hypot_returns_exact_results_for_pythagorean_triples() {
  assert_eq!(hypot(3.0, 4.0), 5.0);
  assert_eq!(hypot(-5.0, 12.0), 13.0);
  assert_eq!(hypot(8.0, -15.0), 17.0);
}

#[test]
fn hypot_returns_positive_zero_for_zeros() {
  let result = hypot(-0.0, -0.0);

  assert_eq!(result, 0.0);
  assert!(result.is_sign_positive());
}

#[test]
fn hypot_prefers_infinity_over_nan() {
  assert_eq!(hypot(f64::NAN, f64::NEG_INFINITY), f64::INFINITY);
  assert_eq!(hypot(f64::INFINITY, f64::NAN), f64::INFINITY);
  assert!(hypot(f64::NAN, 1.0).is_nan());
}

#[test]
fn hypot_does_not_overflow_or_underflow_in_between() {
  assert_eq!(hypot(1e308, 1e308), 1.414_213_562_373_095_1e308);
  // Three, four and five times the smallest subnormal number.
  assert_eq!(
    hypot(f64::from_bits(3), f64::from_bits(4)),
    f64::from_bits(5)
  );
  assert_eq!(hypot(f64::MIN_POSITIVE, 0.0), f64::MIN_POSITIVE);
  assert_eq!(hypot(f64::MAX, f64::MAX), f64::INFINITY);
}
